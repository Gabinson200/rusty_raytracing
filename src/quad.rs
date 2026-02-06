// quad.rs

use std::sync::Arc;
use crate::vec3::{Point3, Vec3};
use crate::material::Material;
use crate::aabb::AABB;
use crate::hittable::{Hittable, HitRecord};
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use crate::ray::Ray;

pub struct Quad {
    q: Point3,
    u: Vec3,
    v: Vec3,
    material: Arc<dyn Material>,
    bbox: AABB,
    normal: Vec3,
    D: f64,
    w: Vec3,
}

impl Quad {
    pub fn new(q: Point3, u: Vec3, v: Vec3, material: Arc<dyn Material>) -> Self {
        let bbox_diagonal1 = AABB::extrema_box(q, q + u + v);
        let bbox_diagonal2 = AABB::extrema_box(q + u, q + v);
        let bbox = AABB::from_two_boxes(bbox_diagonal1, bbox_diagonal2);

        let n = u.cross(v);
        let normal = n.unit_vector();
        let D = normal.dot(q);
        let w = n / n.dot(n);

        Self { q, u, v, material, bbox, normal, D, w }
    }


    pub fn is_interior(&self, alpha: f64, beta: f64, rec: &mut HitRecord) -> bool {
        let unit_interval = Interval::new(0.0, 1.0);
        if !unit_interval.contains(alpha) || !unit_interval.contains(beta) {
            return false;
        }

        // for triangle {;
        /* 
        if alpha < 0.0 || beta < 0.0 || (alpha + beta) > 1.0 {
            return false;
        }
        */

        // for ellipses (:
        /* 
        if !unit_interval.contains(alpha) || !unit_interval.contains(beta) || (alpha*alpha + beta*beta).sqrt() > 1.0 {
            return false;
        }
        */

        rec.u = alpha;
        rec.v = beta;
        return true;
    }

    #[inline]
    pub fn make_box(a: &Point3, b: &Point3, material: Arc<dyn Material>) -> HittableList {
        let mut sides = HittableList::new();

        // two opposite vertices with min and max coordinates
        let min = Point3::new(a.x().min(b.x()), a.y().min(b.y()), a.z().min(b.z()));
        let max = Point3::new(a.x().max(b.x()), a.y().max(b.y()), a.z().max(b.z()));

        let dx = Vec3::new(max.x() - min.x(), 0.0, 0.0);
        let dy = Vec3::new(0.0, max.y() - min.y(), 0.0);
        let dz = Vec3::new(0.0, 0.0, max.z() - min.z());

        sides.add(Box::new(Quad::new(Point3::new(min.x(), min.y(), max.z()), dx, dy, material.clone()))); //front
        sides.add(Box::new(Quad::new(Point3::new(max.x(), min.y(), max.z()), -dz, dy, material.clone()))); // right
        sides.add(Box::new(Quad::new(Point3::new(max.x(), min.y(), min.z()), -dx, dy, material.clone()))); // back
        sides.add(Box::new(Quad::new(Point3::new(min.x(), min.y(), min.z()), dz, dy, material.clone()))); // left
        sides.add(Box::new(Quad::new(Point3::new(min.x(), max.y(), max.z()), dx, -dz, material.clone()))); // top
        sides.add(Box::new(Quad::new(Point3::new(min.x(), min.y(), min.z()), dx, dz, material.clone()))); // bottom

        return sides
    }
}

impl Hittable for Quad {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let denom = self.normal.dot(r.direction());

        // no hit if plane is parallel to ray
        if denom.abs() < 1e-8 {
            return false;
        }

        // return false if the hit point parameter t is outside of the ray interval
        let t = (self.D - self.normal.dot(r.origin())) / denom;
        if !ray_t.contains(t) {
            return false;
        }

        // Determine if the hitpoint lies within the planar shape using its plane coordinates
        let intersection = r.at(t);
        let planar_hitpt_vector = intersection - self.q;
        let alpha = self.w.dot(planar_hitpt_vector.cross(self.v));
        let beta = self.w.dot(self.u.cross(planar_hitpt_vector));

        if !self.is_interior(alpha, beta, rec){
            return false;
        }

        rec.t = t;
        rec.p = intersection;
        rec.material = self.material.clone();
        rec.set_face_normal(r, self.normal);

        return true;

    }


    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
