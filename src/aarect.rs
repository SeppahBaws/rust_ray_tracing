use std::rc::Rc;

use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    materials::Material,
    vec3::{Point3, Vec3},
};

pub struct XYRect<M: Material> {
    material: M,
    x0: f32,
    x1: f32,
    y0: f32,
    y1: f32,
    k: f32,
}

impl<M: Material> XYRect<M> {
    pub fn new(x0: f32, x1: f32, y0: f32, y1: f32, k: f32, material: M) -> Self {
        Self {
            material,
            x0,
            x1,
            y0,
            y1,
            k,
        }
    }
}

impl<M: Material> Hittable for XYRect<M> {
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        t_min: f32,
        t_max: f32,
    ) -> Option<crate::hittable::HitRecord> {
        let t = (self.k - ray.origin.z) / ray.direction.z;

        if t < t_min || t > t_max {
            return None;
        }

        let x = ray.origin.x + t * ray.direction.x;
        let y = ray.origin.y + t * ray.direction.y;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }

        let mut rec = HitRecord {
            t,
            p: ray.at(t),
            uv: (
                (x - self.x0) / (self.x1 - self.x0),
                (y - self.y0) / (self.y1 - self.y0),
            ),
            mat: &self.material,

            // These two are set with set_face_normal
            normal: Vec3::from(0.0),
            front_face: false,
        };

        rec.set_face_normal(ray, &Vec3::new(0.0, 0.0, 1.0));

        Some(rec)
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<crate::aabb::AABB> {
        Some(AABB::new(
            Point3::new(self.x0, self.y0, self.k - 0.0001),
            Point3::new(self.x1, self.y1, self.k + 0.0001),
        ))
    }
}

pub struct XZRect<M: Material> {
    material: M,
    x0: f32,
    x1: f32,
    z0: f32,
    z1: f32,
    k: f32,
}

impl<M: Material> XZRect<M> {
    pub fn new(x0: f32, x1: f32, z0: f32, z1: f32, k: f32, material: M) -> Self {
        Self {
            material,
            x0,
            x1,
            z0,
            z1,
            k,
        }
    }
}

impl<M: Material> Hittable for XZRect<M> {
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        t_min: f32,
        t_max: f32,
    ) -> Option<crate::hittable::HitRecord> {
        let t = (self.k - ray.origin.y) / ray.direction.y;

        if t < t_min || t > t_max {
            return None;
        }

        let x = ray.origin.x + t * ray.direction.x;
        let z = ray.origin.z + t * ray.direction.z;
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }

        let mut rec = HitRecord {
            t,
            p: ray.at(t),
            uv: (
                (x - self.x0) / (self.x1 - self.x0),
                (z - self.z0) / (self.z1 - self.z0),
            ),
            mat: &self.material,

            // These two are set with set_face_normal
            normal: Vec3::from(0.0),
            front_face: false,
        };

        rec.set_face_normal(ray, &Vec3::new(0.0, 1.0, 0.0));

        Some(rec)
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<crate::aabb::AABB> {
        Some(AABB::new(
            Point3::new(self.x0, self.k - 0.0001, self.z0),
            Point3::new(self.x1, self.k + 0.0001, self.z1),
        ))
    }
}

pub struct YZRect<M: Material> {
    material: M,
    y0: f32,
    y1: f32,
    z0: f32,
    z1: f32,
    k: f32,
}

impl<M: Material> YZRect<M> {
    pub fn new(y0: f32, y1: f32, z0: f32, z1: f32, k: f32, material: M) -> Self {
        Self {
            material,
            y0,
            y1,
            z0,
            z1,
            k,
        }
    }
}

impl<M: Material> Hittable for YZRect<M> {
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        t_min: f32,
        t_max: f32,
    ) -> Option<crate::hittable::HitRecord> {
        let t = (self.k - ray.origin.x) / ray.direction.x;

        if t < t_min || t > t_max {
            return None;
        }

        let y = ray.origin.y + t * ray.direction.y;
        let z = ray.origin.z + t * ray.direction.z;
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None;
        }

        let mut rec = HitRecord {
            t,
            p: ray.at(t),
            uv: (
                (y - self.y0) / (self.y1 - self.y0),
                (z - self.z0) / (self.z1 - self.z0),
            ),
            mat: &self.material,

            // These two are set with set_face_normal
            normal: Vec3::from(0.0),
            front_face: false,
        };

        rec.set_face_normal(ray, &Vec3::new(1.0, 0.0, 0.0));

        Some(rec)
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<crate::aabb::AABB> {
        Some(AABB::new(
            Point3::new(self.k - 0.0001, self.y0, self.z0),
            Point3::new(self.k + 0.0001, self.y1, self.z1),
        ))
    }
}
