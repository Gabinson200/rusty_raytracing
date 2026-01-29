use std::io::{self, Write};
// Import modules
use rusty_raytracing::utils::prelude::*;

fn main() {
    // World
    let mut world = HittableList::new();
    let sphere1 = Sphere::new(Point3::new(-0.4, 0.0, -1.5), 0.5);
    let sphere2 = Sphere::new(Point3::new(0.5, 0.0, -1.0), 0.5);
    let earth = Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0);
    world.add(Box::new(sphere1));
    world.add(Box::new(sphere2));
    world.add(Box::new(earth));

    // Camera
    let mut camera = Camera::new();

    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;

    camera.render(&world);

    //eprintln!("Image dimensions: {}x{}\n", camera.image_width, camera.image_height);
}

