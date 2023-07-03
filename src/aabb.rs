use crate::{
    ray::Ray,
    utils::{max, min},
    vec3::Point3,
};

#[derive(Copy, Clone)]
pub struct AABB {
    pub minimum: Point3,
    pub maximum: Point3,
}

impl AABB {
    pub fn empty() -> Self {
        Self {
            minimum: Point3::from(0.0),
            maximum: Point3::from(0.0),
        }
    }
    pub fn new(minimum: Point3, maximum: Point3) -> Self {
        Self { minimum, maximum }
    }

    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> bool {
        for (minimum, maximum, origin, direction) in [
            (
                self.minimum.x,
                self.maximum.x,
                ray.origin.x,
                ray.direction.x,
            ),
            (
                self.minimum.y,
                self.maximum.y,
                ray.origin.y,
                ray.direction.y,
            ),
            (
                self.minimum.z,
                self.maximum.z,
                ray.origin.z,
                ray.direction.z,
            ),
        ] {
            let inv_d = 1.0 / direction;
            let mut t0 = (minimum - origin) * inv_d;
            let mut t1 = (maximum - origin) * inv_d;

            if inv_d < 0.0 {
                let temp = t0;
                t0 = t1;
                t1 = temp;
            }

            let t_min = if t0 > t_min { t0 } else { t_min };
            let t_max = if t1 < t_max { t1 } else { t_max };

            if t_max <= t_min {
                return false;
            }
        }
        true
    }

    pub fn surrounding_box(box0: &AABB, box1: &AABB) -> Self {
        let minimum = Point3::new(
            min(box0.minimum.x, box1.minimum.x),
            min(box0.minimum.y, box1.minimum.y),
            min(box0.minimum.z, box1.minimum.z),
        );
        let maximum = Point3::new(
            max(box0.maximum.x, box1.maximum.x),
            max(box0.maximum.y, box1.maximum.y),
            max(box0.maximum.z, box1.maximum.z),
        );

        Self { minimum, maximum }
    }
}
