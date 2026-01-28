use std::io::{self, Write};
// Import modules
use rusty_raytracing::utils::prelude::*;

fn ray_color(r:&Ray, world:&dyn Hittable) -> Color {
    let mut rec = HitRecord::new();
    if(world.hit(r, 0.0, INFINITY, &mut rec)) {
        return (rec.normal + Color::new(1.0, 1.0, 1.0)) / 2.0;
    }

    let unit_direction = r.direction().unit_vector();
    let t = (unit_direction.y() + 1.0)/2.0;
    return Color::new(1.0, 1.0, 1.0)*(1.0 - t) + Color::new(0.5, 0.7, 1.0)*t;
}

/*
fn hit_sphere(center:Point3, radius:f64, r:&Ray) -> f64 {
    let oc = center - r.origin();
    let a = r.direction().length().powi(2);
    let h = oc.dot(r.direction());
    let c = oc.length().powi(2) - radius*radius;
    let discriminant = h*h - a*c;
    
    if discriminant < 0.0 {
        return -1.0;
    }else{
        return (h - discriminant.sqrt()) / a;
    }
}
*/

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let mut image_width:i32 = 400;

    // Calculate image height based on aspect ratio and make sure its at least one
    let mut image_height = (image_width as f64 / aspect_ratio) as i32;
    image_height = if image_height < 1 { 1 } else { image_height };
    
    // World
    let mut world = HittableList::new();
    let sphere1 = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5);
    let earth = Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0);
    world.add(Box::new(sphere1));
    world.add(Box::new(earth));

    eprintln!("Image dimensions: {}x{}\n", image_width, image_height);

    // Camera
    let focal_length = 1.0;
    let vp_height = 2.0;
    let vp_width = aspect_ratio * (image_width as f64 / image_height as f64);
    let camera_center = Point3::init_zero();

    // Calculate the vectors across the horizontal and down vertical
    // viewport edges
    let vp_h = Vec3::new(vp_width, 0.0, 0.0);
    let vp_v = Vec3::new(0.0, -vp_height, 0.0);

    // Calculate the h and v delta vectors between pixels
    let pixel_delta_h = vp_h / image_width as f64;
    let pixel_delta_v = vp_v / image_height as f64;

    // Calculate the location of the upper left pixel
    let vp_origin = camera_center - Vec3::new(0.0, 0.0, focal_length)
        - (vp_h / 2.0)
        - (vp_v / 2.0);

    let pixel_origin = vp_origin + (pixel_delta_h + pixel_delta_v)/2.0;


    println!("P3\n{image_width} {image_height}\n255");

    // Render
    for j in 0..image_height {
        eprint!("\rScanlines remaining: {:4}", image_height - j);
        io::stdout().flush().unwrap();
        for i in 0..image_width {
            let pixel_center = pixel_origin + (pixel_delta_h*i) + (pixel_delta_v*j);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(&r, &world);
            Color::write_color(pixel_color);
        }
    }
    eprint!("\rDone.");


}

