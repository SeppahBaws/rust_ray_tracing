use crate::vec3::Color;

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

    pub fn write_color(&mut self, x: u32, y: u32, color: &Color) {
        let ir: u8 = (255.999 * color.x) as u8;
        let ig: u8 = (255.999 * color.y) as u8;
        let ib: u8 = (255.999 * color.z) as u8;

        let idx = (((self.height - y - 1) * self.width + x) * self.nr_channels) as usize;
        self.buffer[idx + 0] = ir;
        self.buffer[idx + 1] = ig;
        self.buffer[idx + 2] = ib;
    }

    pub fn get_pixels(&self) -> &[u8] {
        &self.buffer
    }
}
