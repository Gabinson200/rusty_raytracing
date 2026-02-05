// sphere.rs
use crate::vec3::{Point3, Vec3};
use crate::ray::Ray;
use crate::hittable::{Hittable, HitRecord};
use crate::interval::Interval;
use crate::material::Material;
use std::sync::Arc;
use crate::aabb::AABB;



pub struct Sphere {
    center: Ray, // if stationary ray direction is all zeros
    radius: f64,
    material: Arc<dyn Material>,
    bbox: AABB,
}

impl Sphere {
    pub fn new(center: Ray, radius: f64, material: Arc<dyn Material>) -> Self {
        let rvec = Point3::new(radius, radius, radius);
        let bbox1 = AABB::extrema_box(center.origin() - rvec, center.origin() + rvec);

        // if the sphere is moving, extend the bounding box to include the position at time=1.0
        if center.direction() != Vec3::init_zero() {
            let bbox2 = AABB::extrema_box(center.at(1.0) - rvec, center.at(1.0) + rvec);
            let bbox_combined = AABB::from_two_boxes(bbox1, bbox2);
            return Sphere { center, radius, material, bbox: bbox_combined };
        }
        Sphere { center, radius, material, bbox: bbox1 }
    }


    pub fn center(&self) -> Ray {
        self.center
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }

    pub fn material(&self) -> &Arc<dyn Material> {
        &self.material
    }

    #[inline]
    fn get_sphere_uv(p: &Point3) -> (f64, f64) {
        let theta = (-p.y()).acos();
        let phi = (-p.z()).atan2(p.x()) + std::f64::consts::PI;
        let u = phi / (2.0 * std::f64::consts::PI);
        let v = theta / std::f64::consts::PI;
        (u, v)
    }

}

impl Hittable for Sphere {
    fn hit(&self, ray:&Ray, ray_t:Interval, rec:&mut HitRecord) -> bool {
        let current_center = self.center.at(ray.time());
        let oc = current_center - ray.origin();
        let dir = ray.direction();
        let a = dir.dot(dir);
        let h = oc.dot(dir);
        let c = oc.dot(oc) - self.radius*self.radius;
        let discriminant = h*h - a*c;
        
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (h-sqrtd) / a;
        if !ray_t.contains(root) {
            root = (h + sqrtd) / a;
            if !ray_t.contains(root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = ray.at(rec.t);
        let outward_normal:Vec3 = (rec.p - current_center) / self.radius;
        rec.set_face_normal(&ray, outward_normal);
        let (u, v) = Sphere::get_sphere_uv(&outward_normal);
        rec.u = u;
        rec.v = v;
        rec.material = Arc::clone(&self.material);

        return true;
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
