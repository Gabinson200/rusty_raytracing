// camera.rs

use std::io::{self, Write};
use crate::vec3::{Point3, Vec3, Color};
use crate::ray::Ray;
use crate::hittable::{Hittable, HitRecord};
use crate::interval::Interval;

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u32,
    image_height: u32,
    center: Point3,
    pixel_origin: Point3,
    pixel_delta_h: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {

    pub fn new() -> Self {
        Self {
            aspect_ratio: 16.0 / 9.0,
            image_width: 400,
            image_height: -1,
            center: Point3::init_zero(),
            pixel_origin: Point3::init_zero(),
            pixel_delta_h: Vec3::init_zero(),
            pixel_delta_v: Vec3::init_zero(),
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
                let pixel_center = self.pixel_origin + (self.pixel_delta_h*i) + (self.pixel_delta_v*j);
                let ray_direction = pixel_center - self.center;
                let r = Ray::new(self.center, ray_direction);

                let pixel_color = self.ray_color(&r, world);
                Color::write_color(pixel_color);
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
        let vp_width = self.aspect_ratio * (self.image_width as f64 / image_height as f64);
        let camera_center = Point3::init_zero();

        // Calculate the vectors across the horizontal and down vertical
        // viewport edges
        let vp_h = Vec3::new(vp_width, 0.0, 0.0);
        let vp_v = Vec3::new(0.0, -vp_height, 0.0);

        // Calculate the h and v delta vectors between pixels
        let pixel_delta_h = vp_h / self.image_width as f64;
        let pixel_delta_v = vp_v / image_height as f64;

        // Calculate the location of the upper left pixel
        let vp_origin = camera_center - Vec3::new(0.0, 0.0, focal_length)
            - (vp_h / 2.0)
            - (vp_v / 2.0);

        let pixel_origin = vp_origin + (pixel_delta_h + pixel_delta_v)/2.0;

        self.image_height = image_height as u32;
        self.center = camera_center;
        self.pixel_origin = pixel_origin;
        self.pixel_delta_h = pixel_delta_h;
        self.pixel_delta_v = pixel_delta_v;
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
