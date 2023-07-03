use image::io::Reader as ImageReader;
use std::rc::Rc;

use crate::{
    perlin::Perlin,
    vec3::{Color, Point3},
};

pub trait Texture {
    fn value(&self, uv: &(f32, f32), p: Point3) -> Color;
}

pub struct SolidColor {
    color: Color,
}

impl SolidColor {
    pub fn new(c: Color) -> Self {
        Self { color: c }
    }
}

impl Texture for SolidColor {
    fn value(&self, uv: &(f32, f32), p: Point3) -> Color {
        self.color
    }
}

pub struct CheckerTexture {
    even: Rc<dyn Texture>,
    odd: Rc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(even: SolidColor, odd: SolidColor) -> Self {
        Self {
            even: Rc::new(even),
            odd: Rc::new(odd),
        }
    }

    pub fn from_textures(even: Rc<dyn Texture>, odd: Rc<dyn Texture>) -> Self {
        Self { even, odd }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, uv: &(f32, f32), p: Point3) -> Color {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();

        if sines < 0.0 {
            self.odd.value(uv, p)
        } else {
            self.even.value(uv, p)
        }
    }
}

pub struct NoiseTexture {
    noise: Perlin,
    scale: f32,
}

impl NoiseTexture {
    pub fn new(scale: f32) -> Self {
        Self {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _uv: &(f32, f32), p: Point3) -> Color {
        let noise = self.scale * p.z + 10.0 * self.noise.turb(&p);
        Color::from(1.0) * 0.5 * (1.0 + noise.sin())
    }
}

pub struct ImageTexture {
    data: Vec<u8>,
    width: u32,
    height: u32,
}

impl ImageTexture {
    pub fn new(img_path: &str) -> Self {
        let img = ImageReader::open(img_path).unwrap().decode().unwrap();

        Self {
            data: img.as_bytes().to_vec(),
            width: img.width(),
            height: img.height(),
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, uv: &(f32, f32), p: Point3) -> Color {
        let u = uv.0.clamp(0.0, 1.0);
        let v = 1.0 - uv.1.clamp(0.0, 1.0);

        let mut i = (u * self.width as f32) as u32;
        let mut j = (v * self.height as f32) as u32;

        if i >= self.width {
            i = self.width - 1;
        }
        if j >= self.height {
            j = self.height - 1;
        }

        let color_scale = 1.0 / 255.0;
        let bytes_per_pixel = 3;
        let bytes_per_scanline = bytes_per_pixel * self.width;
        let pixel_idx = j * bytes_per_scanline + i * bytes_per_pixel;
        let pixel_r = self.data[(pixel_idx + 0) as usize] as f32 * color_scale;
        let pixel_g = self.data[(pixel_idx + 1) as usize] as f32 * color_scale;
        let pixel_b = self.data[(pixel_idx + 2) as usize] as f32 * color_scale;

        Color::new(pixel_r, pixel_g, pixel_b)

        // Color::from(v)
    }
}
