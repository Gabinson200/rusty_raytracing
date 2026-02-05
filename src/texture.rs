// texture.rs

use std::sync::Arc;

use crate::vec3::Color;
use crate::vec3::Point3;

pub trait Texture{
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;  
}

// Solid color texture

#[derive(Copy, Clone, Debug)]
pub struct SolidColor{
    albedo: Color
}

impl SolidColor{
    pub fn new(albedo: Color) -> Self {
        SolidColor { albedo }
    }

    pub fn from_rgb(r: f64, g: f64, b: f64) -> Self {
        SolidColor { albedo: Color::new(r, g, b) }
    }

}

impl Texture for SolidColor{
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        self.albedo
    }
}

// Checker texture

#[derive(Clone)]
pub struct CheckerTexture{
    inv_scale: f64,
    even: Arc<dyn Texture>,
    odd: Arc<dyn Texture>,
}

impl CheckerTexture{
    pub fn new(scale: f64, even: Arc<dyn Texture>, odd: Arc<dyn Texture>) -> Self {
        CheckerTexture { inv_scale: 1.0/scale, even, odd }
    }

    pub fn from_colors(scale: f64, color1: Color, color2: Color) -> Self {
        Self::new(
            scale,
            Arc::new(SolidColor::new(color1)),
            Arc::new(SolidColor::new(color2)),
        )
    }
}

impl Texture for CheckerTexture{
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        let x_int = (self.inv_scale * p.x()).floor() as i32;
        let y_int = (self.inv_scale * p.y()).floor() as i32;
        let z_int = (self.inv_scale * p.z()).floor() as i32;

        let isEven = (x_int + y_int + z_int) % 2 == 0;

        match isEven {
            true => self.even.value(u, v, p),
            false => self.odd.value(u, v, p),
        }
    }
}




