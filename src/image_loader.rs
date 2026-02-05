// image_loader.rs

use image::ImageReader as ImageReader;
use image::GenericImageView;
use crate::Color;

pub struct ImageTextureData {
    pub width: u32,
    pub height: u32,
    pub rgb: Vec<u8>, // 3 bytes per pixel: R,G,B
}

impl ImageTextureData{

    pub fn load_rgb8(path: &str) -> Result<ImageTextureData, image::ImageError> {
        let img = ImageReader::open(path)?.decode()?;     // decode jpg/png/etc.
        let rgb_img = img.to_rgb8();                      // force RGB8
        let (w, h) = rgb_img.dimensions();
        Ok(ImageTextureData {
            width: w,
            height: h,
            rgb: rgb_img.into_raw(),                      // contiguous RGB buffer
        })
    }

    #[inline]
    fn srgb_to_linear(c: f64) -> f64 {
        if c <= 0.04045 {
            c / 12.92
        } else {
            ((c + 0.055) / 1.055).powf(2.4)
        }
    }

    pub fn pixel_data(image: &ImageTextureData, x: u32, y: u32) -> Color {
        let color = Color::new(1.0, 0.0,1.0); // magenta for out-of-bounds
        if x >= image.width || y >= image.height {
            return color;
        }
        let idx = ((y * image.width + x) * 3) as usize;
        let r = image.rgb[idx] as f64 / 255.0;
        let g = image.rgb[idx + 1] as f64 / 255.0;
        let b = image.rgb[idx + 2] as f64 / 255.0;
        Color::new(Self::srgb_to_linear(r), Self::srgb_to_linear(g), Self::srgb_to_linear(b))
    }

    fn float_to_byte(value: f64) -> u8 {
        if value < 0.0 {
            return 0 as u8;
        } else if value >= 1.0 {
            return 255 as u8;
        }
        return (value * 256.0) as u8;
    }
}

