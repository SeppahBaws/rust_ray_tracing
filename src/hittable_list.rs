use std::rc::Rc;

use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    ray::Ray,
};

pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn from(obj: Rc<dyn Hittable>) -> Self {
        let mut this = Self {
            objects: Vec::new(),
        };

        this.objects.push(obj);

        this
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, obj: Rc<dyn Hittable>) {
        self.objects.push(obj);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut hit_anything: Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            match object.hit(ray, t_min, closest_so_far) {
                Some(hit) => {
                    closest_so_far = hit.t;
                    hit_anything = Some(hit);
                }
                _ => (),
            }
        }

        hit_anything
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        if self.objects.is_empty() {
            return None;
        }

        let temp_box = AABB::empty();
        let mut output_box = AABB::empty();
        let mut first_box = true;

        for object in self.objects.iter() {
            if object.bounding_box(t0, t1).is_none() {
                return None;
            }

            output_box = if first_box {
                temp_box.clone()
            } else {
                AABB::surrounding_box(&output_box, &temp_box)
            };
            first_box = false;
        }

        Some(output_box)
    }
}
