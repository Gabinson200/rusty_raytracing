// hittable.rs

use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use crate::interval::Interval;

#[derive(Clone, Copy, Debug)]
pub struct HitRecord {
    pub p:Point3,
    pub normal:Vec3,
    pub t:f64,
    pub front_face:bool,
}

impl HitRecord {
    pub fn new() -> Self {
        Self {
            p: Point3::init_zero(),
            normal: Vec3::init_zero(),
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
    fn hit(&self, r:&Ray, interval:Interval, rec:&mut HitRecord) -> bool{
        println!("Hittable hit called on not implemented object");
        return false
    }
}
