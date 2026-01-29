// camera.rs

use std::io::{self, Write};
use crate::vec3::{Point3, Vec3, Color};
use crate::ray::Ray;
use crate::hittable::{Hittable, HitRecord};
use crate::interval::Interval;
use crate::utils::prelude::random_f64;

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub samples_per_pixel: u32,

    image_height: u32,
    center: Point3,
    pixel_origin: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel_samples_scale: f64,
}

impl Camera {

    pub fn new() -> Self {
        Self {
            aspect_ratio: 16.0 / 9.0,
            image_width: 400,
            image_height: 0,
            samples_per_pixel: 100,
            center: Point3::init_zero(),
            pixel_origin: Point3::init_zero(),
            pixel_delta_u: Vec3::init_zero(),
            pixel_delta_v: Vec3::init_zero(),
            pixel_samples_scale: 0.0,
        }
    }

    pub fn render(&mut self, world: &impl Hittable){
        // Initialize camera parameters
        self.initialize();

        println!("P3\n{} {}\n255", self.image_width, self.image_height);

        // Render
        for j in 0..self.image_height {
            eprint!("\rScanlines remaining: {:4}", self.image_height - j);
            io::stdout().flush().unwrap();
            for i in 0..self.image_width {
                let mut pixel_color = Color::init_zero();
                for sample in 0..self.samples_per_pixel {
                    let r: Ray = self.get_ray(i, j);
                    pixel_color = pixel_color + self.ray_color(&r, world);
                }
                
                Color::write_color(pixel_color * self.pixel_samples_scale);
            }
        }
        eprint!("\rDone.");
    }

    fn initialize(&mut self) {
        // Calculate image height based on aspect ratio and make sure its at least one
        let mut image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        image_height = if image_height < 1 { 1 } else { image_height };
    
        // Camera
        let focal_length = 1.0;
        let vp_height = 2.0;
        let vp_width = self.aspect_ratio * vp_height;
        let camera_center = Point3::init_zero();

        // Calculate the vectors across the horizontal and down vertical
        // viewport edges
        let vp_h = Vec3::new(vp_width, 0.0, 0.0);
        let vp_v = Vec3::new(0.0, -vp_height, 0.0);

        // Calculate the h and v delta vectors between pixels
        let pixel_delta_u = vp_h / self.image_width as f64;
        let pixel_delta_v = vp_v / image_height as f64;

        // Calculate the location of the upper left pixel
        let vp_origin = camera_center - Vec3::new(0.0, 0.0, focal_length)
            - (vp_h / 2.0)
            - (vp_v / 2.0);

        let pixel_origin = vp_origin + (pixel_delta_u + pixel_delta_v)/2.0;

        self.image_height = image_height as u32;
        self.center = camera_center;
        self.pixel_origin = pixel_origin;
        self.pixel_delta_u = pixel_delta_u;
        self.pixel_delta_v = pixel_delta_v;
        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;
    }

    fn sample_square(&self) -> Vec3 {
        // Returns the vector to a random point in the [-.5, -.5] to [.5, .5] unit square
        return Point3::new(random_f64() - 0.5, random_f64() - 0.5, 0.0);

    }

    fn get_ray(&self, i:u32, j:u32) -> Ray {
        let offset: Point3 = self.sample_square();
        //let pixel_sample: Point3 = Point3::new(self.pixel_origin.x() + (i as f64 + offset.x()) * self.pixel_delta_u.x(),
        //                                       self.pixel_origin.y() + (j as f64 + offset.y()) * self.pixel_delta_u.y(),
        //                                       self.pixel_origin.z());
        let pixel_sample: Point3 = self.pixel_origin
            + (self.pixel_delta_u * (i as f64 + offset.x()))
            + (self.pixel_delta_v * (j as f64 + offset.y()));
        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;
        return Ray::new(ray_origin, ray_direction);
    }

    fn ray_color(&self, r: &Ray, world: &impl Hittable) -> Color {
        let mut rec = HitRecord::new();

        if(world.hit(r, Interval::new(0.0, f64::INFINITY), &mut rec)) {
            return (rec.normal + Color::new(1.0, 1.0, 1.0)) / 2.0;
        }

        let unit_direction: Vec3 = r.direction().unit_vector();
        let a = (unit_direction.y() + 1.0) / 2.0;
        return Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a;
    }

}
