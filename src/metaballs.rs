// metaballs.rs
use std::sync::Arc;

use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

#[derive(Clone)]
pub struct Metaballs {
    centers: Vec<Point3>,
    radii: Vec<f64>,
    threshold: f64,
    material: Arc<dyn Material + Send + Sync>,
    bbox: AABB,
}

impl Metaballs {
    pub fn new(
        centers: Vec<Point3>,
        radii: Vec<f64>,
        threshold: f64,
        material: Arc<dyn Material + Send + Sync>,
    ) -> Self {
        assert_eq!(centers.len(), radii.len());
        let n = centers.len() as f64;

        // Field: sum(r^2 / d^2). For a single ball, iso-surface at threshold is d = r/sqrt(threshold).
        // For multiple balls, a conservative bbox inflate is sqrt(N/threshold).
        let thresh = threshold.max(1e-12);
        let inflate = (n / thresh).sqrt();

        let mut bbox = AABB::empty();
        for (c, r) in centers.iter().zip(radii.iter()) {
            let re = r * inflate;
            let rad = Vec3::new(re, re, re);
            let b = AABB::extrema_box(*c - rad, *c + rad);
            bbox = AABB::from_two_boxes(bbox, b);
        }

        Self {
            centers,
            radii,
            threshold,
            material,
            bbox,
        }
    }

    #[inline]
    fn field(&self, p: Point3) -> f64 {
        let mut sum = 0.0;
        for (c, r) in self.centers.iter().zip(self.radii.iter()) {
            let d2 = (p - *c).length_squared().max(1e-12);
            sum += (r * r) / d2;
        }
        sum
    }

    // Gradient of `field(p)` (points toward *increasing* field, i.e., generally INWARD).
    // Outward normal for iso-surface of F=field-threshold is -grad(field).
    #[inline]
    fn grad_field(&self, p: Point3) -> Vec3 {
        let mut g = Vec3::init_zero();
        for (c, r) in self.centers.iter().zip(self.radii.iter()) {
            let v = p - *c;
            let d2 = v.length_squared().max(1e-12);
            // d/dp (r^2 / d2) = -2 r^2 * v / d2^2
            let s = -2.0 * (r * r) / (d2 * d2);
            g = g + v * s;
        }
        g
    }

    // Returns (F, dF/dt) where F(t) = field(r(t)) - threshold.
    #[inline]
    fn field_and_dfdt(&self, p: Point3, dir: Vec3) -> (f64, f64) {
        let mut field = 0.0;
        let mut dfdt = 0.0;

        for (c, r) in self.centers.iter().zip(self.radii.iter()) {
            let v = p - *c;
            let d2 = v.length_squared().max(1e-12);

            let r2 = r * r;
            field += r2 / d2;

            // grad(field) contribution: -2 r^2 * v / d2^2
            // df/dt = grad · dir
            let inv_d4 = 1.0 / (d2 * d2);
            let s = -2.0 * r2 * inv_d4;
            dfdt += (v.dot(dir)) * s;
        }

        (field - self.threshold, dfdt)
    }

    #[inline]
    fn bbox_diag(&self) -> f64 {
        let dx = self.bbox.x.size();
        let dy = self.bbox.y.size();
        let dz = self.bbox.z.size();
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    #[inline]
    fn outward_normal(&self, p: Point3, ray_dir: Vec3) -> Vec3 {
        // IMPORTANT: outward for F=field-threshold is -grad(field)
        let g = self.grad_field(p);
        let n = (-g); // flip sign to make it OUTWARD
        let ls = n.length_squared();
        if ls > 1e-24 {
            n / ls.sqrt()
        } else {
            // fallback: still provide a sane normal
            (-ray_dir).unit_vector()
        }
    }
}

impl Hittable for Metaballs {
    fn hit<'a>(&'a self, r: &Ray, ray_t: Interval, rec: &mut HitRecord<'a>) -> bool {
        // Restrict marching to where ray intersects metaballs' bbox
        let Some(bb_t) = self.bbox.hit_interval(r, ray_t) else {
            return false;
        };

        let diag = self.bbox_diag().max(1e-6);

        // Tuned for Cornell-scale and also safe for constant_medium second-hit queries.
        // (Key: eps should be small enough relative to the "rec1.t + 0.0001" offset.)
        let dt_max = (0.02 * diag).clamp(0.2, 3.0);
        let dt_min = (0.05 * dt_max).clamp(0.002, 0.2);
        let eps    = (2e-6 * diag).clamp(1e-4, 2e-3); // << smaller than before
        let safety = 0.7;
        let max_steps = 20000;

        let dir = r.direction();

        let mut t = bb_t.min;

        // Compute F at start
        let (mut f, mut dfdt) = self.field_and_dfdt(r.at(t), dir);

        // ---- constant_medium robustness: avoid immediately "rehitting" the same surface ----
        // If we're already near the surface at the entry of the interval (common on 2nd hit),
        // nudge forward by a small amount so we can find the *next* crossing.
        if f.abs() < eps {
            t = (t + (dt_min.max(10.0 * eps))).min(bb_t.max);
            if t >= bb_t.max {
                return false;
            }
            let tmp = self.field_and_dfdt(r.at(t), dir);
            f = tmp.0;
            dfdt = tmp.1;
        }

        for _ in 0..max_steps {
            if t >= bb_t.max {
                break;
            }

            // Newton-ish distance estimate: dt ~ |F| / |dF/dt|
            let denom = dfdt.abs().max(1e-12);
            let mut dt = safety * (f.abs() / denom);
            dt = dt.clamp(dt_min, dt_max);

            let t_next = (t + dt).min(bb_t.max);
            let (f_next, dfdt_next) = self.field_and_dfdt(r.at(t_next), dir);

            // If we bracketed a root, refine it
            if (f > 0.0 && f_next <= 0.0) || (f < 0.0 && f_next >= 0.0) {
                let mut a = t;
                let mut b = t_next;
                let mut fa = f;

                // Bisection
                for _ in 0..30 {
                    let m = 0.5 * (a + b);
                    let (fm, _) = self.field_and_dfdt(r.at(m), dir);
                    if fm.abs() < eps {
                        a = m;
                        b = m;
                        break;
                    }
                    if (fa > 0.0 && fm > 0.0) || (fa < 0.0 && fm < 0.0) {
                        a = m;
                        fa = fm;
                    } else {
                        b = m;
                    }
                }

                let thit = 0.5 * (a + b);
                let phit = r.at(thit);

                rec.t = thit;
                rec.p = phit;
                rec.material = Some(&*self.material);

                let outward = self.outward_normal(phit, dir);
                rec.set_face_normal(r, outward);
                return true;
            }

            // Advance
            t = t_next;
            f = f_next;
            dfdt = dfdt_next;
        }

        false
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
