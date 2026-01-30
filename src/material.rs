// material.rs

use crate::vec3::{Color, Vec3, Point3};
use crate::hittable::{HitRecord};
use crate::ray::Ray;
use crate::utils::prelude::{random_f64, random_f64_range};

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool{
        return false;
    }
}


// Lambertian diffuse material
pub struct Lambertian {
    albedo: Color
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

// Metal material
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        // max fuzz is 1.0
        Self { albedo, fuzz: if fuzz < 1.0 { fuzz } else { 1.0 } }
    }
}

// Dielectric material
pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }

    pub fn reflectance(&self, cosine: f64, ref_idx: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        return r0 + (1.0 - r0) * (1.0 - cosine).powi(5);
    }
}


// Material implementations
impl Material for Lambertian {

    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;
        return true;
    }
}

impl Material for Metal {

    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let mut reflected = Vec3::reflect(r_in.direction(), rec.normal);
        // Add fuzziness to the reflection
        reflected = reflected.unit_vector() + (Vec3::random_unit_vector() * self.fuzz);
        *scattered = Ray::new(rec.p, reflected);
        *attenuation = self.albedo;
        return scattered.direction().dot(rec.normal) > 0.0;
    }
}

impl Material for Dielectric {

    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_index = if rec.front_face { 1.0 / self.refraction_index } else { self.refraction_index };

        let unit_direction = r_in.direction().unit_vector();
        let cos_theta = f64::min((-unit_direction).dot(rec.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_index * sin_theta > 1.0;
        let mut direction = Vec3::new(0.0, 0.0, 0.0);

        if (cannot_refract || self.reflectance(cos_theta, refraction_index) > random_f64()) {
            direction = Vec3::reflect(unit_direction, rec.normal);
        }else{
            direction = Vec3::refract(&unit_direction, &rec.normal, refraction_index);
        }
       
        *scattered = Ray::new(rec.p, direction);
        return true;
    }
}
