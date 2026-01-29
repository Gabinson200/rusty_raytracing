// utils.rs

pub mod prelude {
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

    // Common constants
    // not needed pub const INFINITY: f64 = f64::INFINITY;
    pub const PI: f64 = std::f64::consts::PI;

    // Common functions
    #[inline]
    pub fn degrees_to_radians(degrees: f64) -> f64 {
        degrees * PI / 180.0
    }
}
