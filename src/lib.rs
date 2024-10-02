pub mod text;
pub mod load_image;
pub mod utility;
pub mod font;
pub mod buffer;
pub mod rect;
pub mod circle;

use std::fs::File;
use std::io::copy;
use std::io::Cursor;

use image::codecs::png::{CompressionType, FilterType};

pub use rusttype;
pub use regex;
pub use ar_reshaper;
pub use image;
pub use libwebp_sys;
pub use png;

#[derive(Clone)]
pub struct Ghe2d {
    pub image:image::RgbaImage,
    width: u32,
    height: u32
}

impl Ghe2d {
    pub fn new(width: u32, height: u32) -> Ghe2d {
        Ghe2d {
            image: image::RgbaImage::new(width, height),
            width,
            height
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn save(&self, path: &str) -> Result<(), image::ImageError> {
        self.image.save(path)
    }

    pub fn save_with_png(&self, path: &str, compression: CompressionType, filter: FilterType) -> Result<(), png::EncodingError> {
        let mut dest = File::create(path)?;
        let mut content = Cursor::new(self.to_png_buffer(compression, filter));
        copy(&mut content, &mut dest)?;
        Ok(())
    }

    pub fn save_with_webp(&self, path: &str, quality: f32) -> Result<(), libwebp_sys::WebPEncodingError> {
        let mut dest = File::create(path).unwrap();
        let mut content = Cursor::new(self.to_webp_buffer(quality)?);
        copy(&mut content, &mut dest).unwrap();
        Ok(())
    }
    
    pub fn to_png_buffer(&self, compression: CompressionType, filter: FilterType) -> Vec<u8> {
        buffer::image_to_png_buffer(&self.image, compression, filter)
    }

    pub fn to_webp_buffer(&self, quality: f32) -> Result<Vec<u8>, libwebp_sys::WebPEncodingError> {
        buffer::image_to_webp_buffer(&self.image, quality)
    }

    pub fn draw_text(&mut self, load_font: font::LoadFont, text: String, x: u32, y: u32, size: u32, color: utility::Rgba) -> &Ghe2d {
        text::draw_text(&mut self.image, load_font, text, x as f32, y as f32, size as f32, color);
        self
    }

    pub fn load_image(&mut self, path: &str, x: u32, y: u32, width: u32, height: u32, is_circle: bool) -> &Ghe2d {
        load_image::add_image_mut(&mut self.image, path, x, y, width, height, is_circle);
        self
    }

    pub fn draw_rect(&mut self, x: u32, y: u32, width: u32, height: u32, color: utility::Rgba) -> &Ghe2d {
        rect::draw_rect(&mut self.image, x, y, width, height, color);
        self
    }

    pub fn draw_circle(&mut self, x: u32, y: u32, raduis: u32, color: utility::Rgba) -> &Ghe2d {
        circle::draw_circle(&mut self.image, x, y, raduis, color);
        self
    }

    pub fn from_buffer(&mut self, buffer: Vec<u8>, x: u32, y: u32, width: u32, height: u32, is_circle: bool) -> &Ghe2d {
        buffer::load_buffer_image(&mut self.image, buffer, x, y, width, height, is_circle);
        self
    }
}
