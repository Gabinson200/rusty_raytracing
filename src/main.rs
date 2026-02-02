use std::io::{self, Write};
use std::sync::Arc;

// Import modules
use rusty_raytracing::utils::prelude::*;

fn main() {

    // World
    let mut world = HittableList::new();

    // Ground
    let material_ground = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::new(Ray::new(Point3::new(0.0, -1000.0, 0.0), Vec3::new(0.0, 0.0, 0.0)), 1000.0, material_ground)));

    // Spheres
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_f64();
            let center:Ray = Ray::new(Point3::new(a as f64 + 0.9*random_f64(), 0.2, b as f64 + 0.9*random_f64()), Vec3::new(0.0, 0.0, 0.0));
            if (center.origin() - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Arc<dyn Material>;
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::new(random_f64()*random_f64(), random_f64()*random_f64(), random_f64()*random_f64());
                    sphere_material = Arc::new(Lambertian::new(albedo));
                    let moving_center: Ray = Ray::new(center.origin(), Vec3::new(0.5, random_f64_range(0.0, 0.2), 0.0));
                    world.add(Box::new(Sphere::new(moving_center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::new(random_f64_range(0.5, 1.0), random_f64_range(0.5, 1.0), random_f64_range(0.5, 1.0));
                    let fuzz = random_f64_range(0.0, 0.5);
                    sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // glass
                    sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Box::new(Sphere::new(Ray::new(Point3::new(0.0, 1.0, 0.0), Vec3::new(0.0, 0.0, 0.0)), 1.0, material1)));

    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new(Ray::new(Point3::new(-4.0, 1.0, 0.0), Vec3::new(0.0, 0.0, 0.0)), 1.0, material2)));

    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(Ray::new(Point3::new(4.0, 1.0, 0.0), Vec3::new(0.0, 0.0, 0.0)), 1.0, material3)));

    // Camera
    let mut camera = Camera::new();

    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;
    camera.vfov = 20.0;
    camera.look_from = Point3::new(13.0, 2.0, 3.0);
    camera.look_at = Point3::new(0.0, 0.0, 0.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);

    camera.defocus_angle = 0.6; // degrees
    camera.focus_distance = 10.0;

    camera.render(&world);

    //eprintln!("Image dimensions: {}x{}\n", camera.image_width, camera.image_height);
}

