// sphere.rs
use crate::vec3::{Point3, Vec3};
use crate::ray::Ray;
use crate::hittable::{Hittable, HitRecord};

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Sphere { center, radius }
    }

    pub fn center(&self) -> Point3 {
        self.center
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray:&Ray, ray_tmin:f64, ray_tmax:f64, rec:&mut HitRecord) -> bool {
        let oc = self.center - ray.origin();
        let a = ray.direction().length().powi(2);
        let h = oc.dot(ray.direction());
        let c = oc.length().powi(2) - self.radius*self.radius;
        let discriminant = h*h - a*c;
        
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (h-sqrtd) / a;
        if root <= ray_tmin || root >= ray_tmax {
            root = (h + sqrtd) / a;
            if root <= ray_tmin || root >= ray_tmax {
                return false;
            }
        }

        rec.t = root;
        rec.p = ray.at(rec.t);
        let outward_normal:Vec3 = (rec.p - self.center) / self.radius;
        rec.set_face_normal(&ray, outward_normal);

        return true;
    }
}
