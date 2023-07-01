use std::rc::Rc;

use crate::vec3::{Color, Point3};

pub trait Texture {
    fn value(&self, uv: (f32, f32), p: Point3) -> Color;
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
    fn value(&self, uv: (f32, f32), p: Point3) -> Color {
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
    fn value(&self, uv: (f32, f32), p: Point3) -> Color {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();

        if sines < 0.0 {
            self.odd.value(uv, p)
        } else {
            self.even.value(uv, p)
        }
    }
}
