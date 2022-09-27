use crate::vec3::{Point3, Vec3};

pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
    pub time: f32,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3, time: f32) -> Self {
        Self { origin, direction, time }
    }

    pub fn at(&self, t: f32) -> Point3 {
        self.origin + t * self.direction
    }
}
