// texture.rs

use std::sync::Arc;

use crate::vec3::Color;
use crate::vec3::Point3;
use crate::image_loader::ImageTextureData;

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

// Image texture

pub struct ImageTexture{
    data: ImageTextureData,
}

impl ImageTexture{
    pub fn new(filename: &str) -> Self {
        let data = ImageTextureData::load_rgb8(filename).expect("Failed to load image");
        ImageTexture { data }
    }
}

impl Texture for ImageTexture{

    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        if self.data.width <= 0 || self.data.height <= 0 {
            return Color::new(0.0, 1.0, 1.0); // cyan for missing texture
        }

        // clamp input texture coordinates to [0,1]x[1,0]
        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0); // flip V to image coordinates

        let i = (u * self.data.width as f64) as u32;
        let j = (v * self.data.height as f64) as u32;
        let pixel = ImageTextureData::pixel_data( &self.data, i, j);
        
        //let color_scale = 1.0 / 255.0;
        Color::new(pixel.x(), pixel.y(), pixel.z())
    }
}




