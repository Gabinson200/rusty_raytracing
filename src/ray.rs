// ray.rs

use crate::vec3::{Point3, Vec3};

#[derive(Copy, Clone)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
    inv_direction: Vec3,
    time: f64,
}

pub fn point_to_ray(point: Point3) -> Ray {
    let direction = Vec3::new(0.0, 0.0, 0.0);
    let inv_direction = Vec3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
    Ray {
        origin: point,
        direction,
        inv_direction,
        time: 0.0,
    }
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        let inv_direction = Vec3::new(1.0 / direction.x(), 1.0 / direction.y(), 1.0 / direction.z());
        Ray { origin, direction, inv_direction, time: 0.0 }
    }

    pub fn new_time(origin: Point3, direction: Vec3, time: f64) -> Ray {
        let inv_direction = Vec3::new(1.0 / direction.x(), 1.0 / direction.y(), 1.0 / direction.z());
        Ray { origin, direction, inv_direction, time }
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn inv_direction(&self) -> Vec3 {
        self.inv_direction
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }

    pub fn time(&self) -> f64 {
        self.time
    }
}
