// hittable.rs

use std::sync::Arc;

use crate::ray::Ray;
use crate::vec3::{Point3, Vec3, Color};
use crate::interval::Interval;
use crate::material::{Material, Lambertian, Metal};

#[derive(Clone)]
pub struct HitRecord {
    pub p:Point3,
    pub normal:Vec3,
    pub material: Arc<dyn Material>,
    pub t:f64,
    pub front_face:bool,
}

impl HitRecord {
    pub fn new() -> Self {
        Self {
            p: Point3::init_zero(),
            normal: Vec3::init_zero(),
            material: Arc::new(Lambertian::new(Color::new(0.0,0.0,0.0))),
            t: 0.0,
            front_face: true,
        }
    }

    pub fn set_face_normal(&mut self, r:&Ray, outward_normal:Vec3) {
        self.front_face = r.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal } else { -outward_normal };
    }
}

pub trait Hittable {
    fn hit(&self, r:&Ray, interval:Interval, rec:&mut HitRecord) -> bool;
}
