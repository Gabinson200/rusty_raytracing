// constant_medium.rs
use std::sync::Arc;
use crate::hittable::{Hittable, HitRecord};
use crate::material::Material;
use crate::ray::Ray;
use crate::interval::Interval;
use crate::aabb::AABB;
use crate::utils::prelude::random_f64;
use crate::vec3::{Vec3, Color, Point3};
use crate::material::Isotropic;


pub struct constant_medium {
    pub boundary: Arc<dyn Hittable>,
    neg_inv_density: f64,
    phase_function: Arc<dyn Material>,
}

impl constant_medium {
    pub fn new(boundary: Arc<dyn Hittable>, density: f64, phase_function: Arc<dyn Material>) -> Self {
        Self { boundary, neg_inv_density: -1.0 / density, phase_function }
    }

    pub fn from_color(boundary: Arc<dyn Hittable>, density: f64, albedo: &Color) -> Self {
        Self { boundary, neg_inv_density: -1.0 / density, phase_function: Arc::new(Isotropic::new(*albedo)) }
    }

    pub fn bounding_box(&self) -> AABB {
        self.boundary.bounding_box()
    }

}


impl Hittable for constant_medium {
    fn hit(&self, r: &Ray, interval: Interval, rec: &mut HitRecord) -> bool {
        let rec1 = &mut HitRecord::new();
        let rec2 = &mut HitRecord::new();

        if !self.boundary.hit(r, Interval::UNIVERSE, rec1){
            return false;
        }

        if !self.boundary.hit(r, Interval::new(rec1.t + 0.0001, f64::INFINITY), rec2) {
            return false;
        }

        if rec1.t < interval.min{ rec1.t = interval.min;}
        if rec2.t > interval.max{ rec2.t = interval.max;}

        if rec1.t >= rec2.t {
            return false;
        }

        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }

        let ray_length = r.direction().length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * random_f64().ln();

        if hit_distance > distance_inside_boundary {
            return false;
        }

        rec.t = rec1.t + hit_distance / ray_length;
        rec.p = r.at(rec.t);

        rec.normal = Vec3::new(1.0, 0.0, 0.0);
        rec.front_face = true;
        rec.material = self.phase_function.clone();

        return true;
    }

}
