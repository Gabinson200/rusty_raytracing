// camera.rs

use std::f64::INFINITY;
use std::io::{self, Write};
use crate::vec3::{Point3, Vec3, Color};
use crate::ray::Ray;
use crate::hittable::{Hittable, HitRecord};
use crate::interval::Interval;
use crate::utils::prelude::{random_f64, degrees_to_radians};
use crate::material::Material;

pub struct Camera {
    pub aspect_ratio: f64, // ratio of image width over height
    pub image_width: u32, // image width in pixels
    pub samples_per_pixel: u32, // number of samples per pixel for anti-aliasing
    pub max_depth: u32, // max recursion depth for ray tracing
    pub vfov: f64, // vertical field of view in degrees

    // Camera orientation
    pub look_from: Point3,
    pub look_at: Point3,
    pub vup: Vec3,
    
    pub defocus_angle: f64, // defocus angle for depth of field effect
    pub focus_distance: f64, // focus distance for depth of field effect

    pub background_color: Color, // Scene background color
    // Camera basis vectors
    u : Vec3,
    v: Vec3,
    w: Vec3,

    image_height: u32,
    center: Point3,
    pixel_origin: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel_samples_scale: f64,
    defocus_disk_u : Vec3,
    defocus_disk_v : Vec3,
}

impl Camera {

    pub fn new() -> Self {
        Self {
            aspect_ratio: 16.0 / 9.0,
            image_width: 400,
            image_height: 0,
            samples_per_pixel: 50,
            max_depth: 50,
            vfov: 90.0,
            defocus_angle: 0.0,
            focus_distance: 10.0,

            look_from: Point3::init_zero(),
            look_at: Point3::new(0.0, 0.0, -1.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            u: Vec3::init_zero(),
            v: Vec3::init_zero(),
            w: Vec3::init_zero(),
            background_color: Color::new(0.5, 0.7, 1.0), // blueish hue

            center: Point3::init_zero(),
            pixel_origin: Point3::init_zero(),
            pixel_delta_u: Vec3::init_zero(),
            pixel_delta_v: Vec3::init_zero(),
            pixel_samples_scale: 0.0,
            defocus_disk_u : Vec3::init_zero(),
            defocus_disk_v : Vec3::init_zero(),
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
                for _sample in 0..self.samples_per_pixel {
                    let r: Ray = self.get_ray(i, j);
                    pixel_color = pixel_color + self.ray_color(&r, self.max_depth, world);
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
        let camera_center = self.look_from;
        let theta = degrees_to_radians(self.vfov);
        let h = (theta / 2.0).tan();
        let vp_height = 2.0 * h * self.focus_distance;
        let vp_width = self.aspect_ratio * vp_height;

        // Calculate camera basis vectors
        self.w = (self.look_from - self.look_at).unit_vector();
        self.u = self.vup.cross(self.w).unit_vector();
        self.v = self.w.cross(self.u);

        // Calculate the vectors across the horizontal and down vertical
        // viewport edges
        let vp_h =  self.u * vp_width;
        let vp_v =  self.v * -vp_height;

        // Calculate the h and v delta vectors between pixels
        let pixel_delta_u = vp_h / self.image_width as f64;
        let pixel_delta_v = vp_v / image_height as f64;

        // Calculate the location of the upper left pixel
        let vp_origin = camera_center - (self.w * self.focus_distance)
            - (vp_h / 2.0)
            - (vp_v / 2.0);

        let pixel_origin = vp_origin + (pixel_delta_u + pixel_delta_v)/2.0;

        // Calculate the camera defocus disk basis vectors.
        let defocus_radius = self.focus_distance * ((self.defocus_angle.to_radians() / 2.0).tan());
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;

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

    fn defocus_disk_sample(&self) -> Point3 {
        // Sample a random point on the defocus disk
        let p = Vec3::random_in_unit_disk();
        return self.center + (self.defocus_disk_u * p.x()) + (self.defocus_disk_v * p.y()); 
    }

    fn get_ray(&self, i:u32, j:u32) -> Ray {
        // Construct a ray originating from the defocus disk
        // and directed at a randomly sampled point
        // around pixel location i, j
        let offset: Point3 = self.sample_square();

        let pixel_sample: Point3 = self.pixel_origin
            + (self.pixel_delta_u * (i as f64 + offset.x()))
            + (self.pixel_delta_v * (j as f64 + offset.y()));
        let ray_origin: Point3 = if self.defocus_angle <= 0.0 {self.center} else {self.defocus_disk_sample()};
        let ray_direction: Vec3 = pixel_sample - ray_origin;
        let ray_time = random_f64();
        return Ray::new_time(ray_origin, ray_direction, ray_time);
    }

    fn ray_color(&self, r: &Ray, max_depth: u32, world: &impl Hittable) -> Color {
        if max_depth <= 0 {
            return Color::init_zero();
        }

        let mut rec = HitRecord::new();

        if !world.hit(r, Interval::new(0.001, f64::INFINITY), &mut rec) {
            return self.background_color;
        }

        let mut scattered = Ray::new(Point3::init_zero(), Vec3::init_zero());
        let mut attentuation = Color::init_zero();
        let mut color_from_emission = rec.material.emitted(rec.u, rec.v, &rec.p);

        if !rec.material.scatter(r, &rec, &mut attentuation, &mut scattered) {
            return color_from_emission;
        }

        let color_from_scatter = attentuation * self.ray_color(&scattered, max_depth - 1, world);
        return color_from_emission + color_from_scatter;
        /* 
        // Check for ray-object intersection make sure that t is >0.001 to avoid shadow acne
        if world.hit(r, Interval::new(0.001, f64::INFINITY), &mut rec) {
            let mut ray = Ray::new(Point3::init_zero(), Vec3::init_zero());
            let mut attenuation = Color::init_zero();
            if rec.material.scatter(r, &rec, &mut attenuation, &mut ray) {
                return attenuation * self.ray_color(&ray, max_depth-1, world);
            }else{
                return Color::init_zero();
            }
        }

        let unit_direction: Vec3 = r.direction().unit_vector();
        let a = (unit_direction.y() + 1.0) / 2.0;
        return Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a;
        */
    }

}
