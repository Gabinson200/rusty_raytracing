// lib.rs

// imports all the modules to main
pub mod vec3;
pub mod ray;
pub mod color;
pub mod hittable;
pub mod sphere;
pub mod hittable_list;
pub mod utils;
pub mod interval;
pub mod camera;
pub mod material;
pub mod aabb;
pub mod bvh;
pub mod texture;
pub mod image_loader;
pub mod perlin;
pub mod quad;
pub mod constant_medium;

pub use crate::utils::prelude::*;
