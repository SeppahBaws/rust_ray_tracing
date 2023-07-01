use core::panic;
use std::{cmp::Ordering, rc::Rc};

use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    hittable_list::HittableList,
    ray::Ray,
};
use rand::Rng;

pub struct BVHNode {
    pub left: Rc<dyn Hittable>,
    pub right: Rc<dyn Hittable>,
    pub bounding_box: AABB,
}

impl BVHNode {
    pub fn new(src_objects: &[Rc<dyn Hittable>], t0: f32, t1: f32) -> Self {
        // Mutable vec of the list of scene objects, so that we can sort them.
        let mut objects: Vec<Rc<dyn Hittable>> =
            src_objects.iter().map(|object| object.clone()).collect();

        let left: Rc<dyn Hittable>;
        let right: Rc<dyn Hittable>;

        let axis: i32 = rand::thread_rng().gen_range(0..2);
        let comparator = if axis == 0 {
            box_x_compare
        } else if axis == 1 {
            box_y_compare
        } else {
            box_z_compare
        };

        if objects.len() == 1 {
            left = objects[0].clone();
            right = objects[0].clone();
        } else if objects.len() == 2 {
            if comparator(&objects[0], &objects[1]) == Ordering::Less {
                left = objects[0].clone();
                right = objects[1].clone();
            } else {
                left = objects[1].clone();
                right = objects[0].clone();
            }
        } else {
            objects.sort_by(comparator);

            let mid = objects.len() / 2;
            left = Rc::new(BVHNode::new(&objects[0..mid], t0, t1));
            right = Rc::new(BVHNode::new(&objects[mid..objects.len()], t0, t1));
        }

        let box_left = left
            .bounding_box(t0, t1)
            .expect("No bounding box in BVHNode constructor!");
        let box_right = right
            .bounding_box(t0, t1)
            .expect("No bounding box in BVHNode constructor!");

        let bounding_box = AABB::surrounding_box(&box_left, &box_right);

        Self {
            left,
            right,
            bounding_box,
        }
    }

    pub fn from(list: &HittableList, t0: f32, t1: f32) -> Self {
        Self::new(&list.objects, t0, t1)
    }
}

impl Hittable for BVHNode {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if !self.bounding_box.hit(&ray, t_min, t_max) {
            return None;
        }

        let hit_left = self.left.hit(&ray, t_min, t_max);
        let hit_right = self.right.hit(&ray, t_min, t_max);

        match (hit_left, hit_right) {
            (Some(x), _) | (_, Some(x)) => Some(x),
            _ => None,
        }
    }
    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(self.bounding_box)
    }
}

fn box_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>, axis: i32) -> Ordering {
    let box_a = a
        .bounding_box(0.0, 0.0)
        .expect("No bounding box in BVHNode constructor!");
    let box_b = b
        .bounding_box(0.0, 0.0)
        .expect("No bounding box in BVHNode constructor!");

    let is_greater = match axis {
        0 => box_a.minimum.x < box_b.minimum.x,
        1 => box_a.minimum.y < box_b.minimum.y,
        2 => box_a.minimum.z < box_b.minimum.z,
        _ => panic!("Axis out of range!"),
    };

    return if is_greater {
        Ordering::Greater
    } else {
        Ordering::Less
    };
}

fn box_x_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 0)
}
fn box_y_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 1)
}
fn box_z_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 2)
}
