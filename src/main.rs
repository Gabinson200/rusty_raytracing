use std::io::{self, Write};
use std::sync::Arc;

// Import modules
use rusty_raytracing::utils::prelude::*;

fn main() {

    // World
    let mut world = HittableList::new();

    // Materials
    let material_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left   = Arc::new(Dielectric::new(1.50));
    let material_bubble = Arc::new(Dielectric::new(1.00 / 1.50));
    let material_right  = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    // Spheres
    let earth = Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, material_ground);
    let sphere1 = Sphere::new(Point3::new(0.0, 0.0, -1.2),  0.5, material_center);
    let sphere2 = Sphere::new(Point3::new(-1.0, 0.0, -1.0),  0.5, material_left);
    let sphere2_inner = Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.4, material_bubble);
    let sphere3 = Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, material_right);

    world.add(Box::new(earth));
    world.add(Box::new(sphere1));
    world.add(Box::new(sphere2));
    world.add(Box::new(sphere2_inner));
    world.add(Box::new(sphere3));

    // Camera
    let mut camera = Camera::new();

    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;
    camera.vfov = 90.0;
    camera.look_from = Point3::new(-1.0, 1.0, 1.0);
    camera.look_at = Point3::new(0.0, 0.0, -1.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);

    camera.defocus_angle = 10.0; // degrees
    camera.focus_distance = 3.4;

    camera.render(&world);

    //eprintln!("Image dimensions: {}x{}\n", camera.image_width, camera.image_height);
}

