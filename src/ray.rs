// ray.rs

use crate::vec3::{Point3, Vec3};

// Ray struct representing a ray in 3D space
#[derive(Copy, Clone)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
    time: f64,
}

// helper function to turn a point3 into a ray with zero direction and time
pub fn point_to_ray(point: Point3) -> Ray {
    Ray {
        origin: point,
        direction: Vec3::new(0.0, 0.0, 0.0),
        time: 0.0,
    }
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray { origin, direction, time: 0.0}
    }

    pub fn new_time(origin: Point3, direction: Vec3, time: f64) -> Ray {
        Ray { origin, direction, time}
    }


    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }

    pub fn time(&self) -> f64 {
        self.time
    }
}
