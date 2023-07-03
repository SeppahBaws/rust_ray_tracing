use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    materials::Material,
    ray::Ray,
    utils::PI,
    vec3::{Point3, Vec3},
};

pub struct Sphere<M: Material> {
    center: Point3,
    radius: f32,
    material: M,
}

impl<M: Material> Sphere<M> {
    pub fn new(center: Point3, radius: f32, material: M) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }

    fn get_uv(p: &Point3) -> (f32, f32) {
        let theta = (-p.y).acos();
        let phi = (-p.z).atan2(p.x) + PI;

        (phi / (2.0 * PI), theta / PI)
    }
}

impl<M: Material> Hittable for Sphere<M> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = Vec3::dot(&oc, &ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearset root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let p = ray.at(root);
        let outward_normal = (p - self.center) / self.radius;
        let mut rec = HitRecord {
            t: root,
            uv: Self::get_uv(&outward_normal),
            p,
            mat: &self.material,
            // These two are set by set_face_normal
            normal: Vec3::from(0.0),
            front_face: true,
        };

        rec.set_face_normal(ray, &outward_normal);

        Some(rec)
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        let output_box = AABB::new(
            self.center - Vec3::from(self.radius),
            self.center + Vec3::from(self.radius),
        );

        Some(output_box)
    }
}

pub struct MovingSphere<M: Material> {
    center0: Point3,
    center1: Point3,
    radius: f32,
    material: M,
    time0: f32,
    time1: f32,
}

impl<M: Material> MovingSphere<M> {
    pub fn new(
        center0: Point3,
        center1: Point3,
        time0: f32,
        time1: f32,
        radius: f32,
        material: M,
    ) -> Self {
        Self {
            center0,
            center1,
            radius,
            material,
            time0,
            time1,
        }
    }
}

pub fn center<M: Material>(sphere: &MovingSphere<M>, time: f32) -> Point3 {
    sphere.center0
        + ((time - sphere.time0) / (sphere.time1 - sphere.time0))
            * (sphere.center1 - sphere.center0)
}

impl<M: Material> Hittable for MovingSphere<M> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - center(&self, ray.time);
        let a = ray.direction.length_squared();
        let half_b = Vec3::dot(&oc, &ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = f32::sqrt(discriminant);

        // Find the nearset root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let p = ray.at(root);
        let outward_normal = (p - center(&self, ray.time)) / self.radius;
        let mut rec = HitRecord {
            t: root,
            uv: (0.0, 0.0),
            p,
            mat: &self.material,
            // These two are set by set_face_normal
            normal: Vec3::from(0.0),
            front_face: true,
        };

        rec.set_face_normal(ray, &outward_normal);

        Some(rec)
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        let box0 = AABB::new(
            self.center0 - Vec3::from(self.radius),
            self.center0 + Vec3::from(self.radius),
        );
        let box1 = AABB::new(
            self.center1 - Vec3::from(self.radius),
            self.center1 + Vec3::from(self.radius),
        );

        let output_box = AABB::surrounding_box(&box0, &box1);
        Some(output_box)
    }
}
