// utils.rs

pub mod prelude {

    //Random
    use rand::Rng;

    use std::sync::Arc;

    // Types
    pub use crate::vec3::{Color, Point3, Vec3};
    pub use crate::ray::Ray;

    // Hittables
    pub use crate::hittable::{HitRecord, Hittable};
    pub use crate::sphere::Sphere;
    pub use crate::hittable_list::HittableList;
    pub use crate::interval::Interval;

    // Camera
    pub use crate::camera::Camera;

    // Materials
    pub use crate::material::{Material, Lambertian, Metal};

    // Common constants
    // not needed pub const INFINITY: f64 = f64::INFINITY;
    pub const PI: f64 = std::f64::consts::PI;

    // Common functions
    #[inline]
    pub fn degrees_to_radians(degrees: f64) -> f64 {
        return degrees * PI / 180.0;
    }

    // Random number functions

    // Random number in [0,1)
    #[inline]
    pub fn random_f64() -> f64 {
        let random_float: f64 = rand::random::<f64>();
        //eprintln!("Random f64: {}", random_float);
        return random_float;
    }

    // Random number in [min,max)
    #[inline]
    pub fn random_f64_range(min: f64, max: f64) -> f64 {
        return min + (max - min) * random_f64();
    }

}
