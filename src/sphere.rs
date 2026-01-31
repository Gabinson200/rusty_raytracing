// sphere.rs
use crate::vec3::{Point3, Vec3};
use crate::ray::Ray;
use crate::hittable::{Hittable, HitRecord};
use crate::interval::Interval;
use crate::material::Material;
use std::sync::Arc;

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Arc<dyn Material>) -> Self {
        Sphere { center, radius, material }
    }

    pub fn center(&self) -> Point3 {
        self.center
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }

    pub fn material(&self) -> &Arc<dyn Material> {
        &self.material
    }

}

impl Hittable for Sphere {
    fn hit(&self, ray:&Ray, ray_t:Interval, rec:&mut HitRecord) -> bool {
        let oc = self.center - ray.origin();
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
        let outward_normal:Vec3 = (rec.p - self.center) / self.radius;
        rec.set_face_normal(&ray, outward_normal);
        rec.material = Arc::clone(&self.material);

        return true;
    }
}
