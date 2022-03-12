use crate::{utils::clamp, vec3::Color};

pub struct OutputBuffer {
    buffer: Vec<u8>,
    width: u32,
    height: u32,
    nr_channels: u32,
}

impl OutputBuffer {
    pub fn new(width: u32, height: u32, nr_channels: u32) -> Self {
        let mut v = Vec::<u8>::new();
        v.resize((width * height * nr_channels) as usize, 0);
        Self {
            buffer: v,
            width,
            height,
            nr_channels,
        }
    }

    pub fn write_color(&mut self, x: u32, y: u32, color: &Color, samples_per_pixel: u32) {
        let mut r = color.x;
        let mut g = color.y;
        let mut b = color.z;

        // Divide the color by the number of samples and gamma-correct for gamma=2.0
        let scale = 1.0 / (samples_per_pixel as f32);
        r = f32::sqrt(scale * r);
        g = f32::sqrt(scale * g);
        b = f32::sqrt(scale * b);

        let idx = (((self.height - y - 1) * self.width + x) * self.nr_channels) as usize;
        self.buffer[idx + 0] = (256.0 * clamp(r, 0.0, 0.999)) as u8;
        self.buffer[idx + 1] = (256.0 * clamp(g, 0.0, 0.999)) as u8;
        self.buffer[idx + 2] = (256.0 * clamp(b, 0.0, 0.999)) as u8;
    }

    pub fn get_pixels(&self) -> &[u8] {
        &self.buffer
    }
}
