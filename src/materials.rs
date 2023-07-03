use std::rc::Rc;

use crate::{
    hittable::HitRecord,
    ray::Ray,
    texture::{SolidColor, Texture},
    utils::random,
    vec3::{Color, Point3, Vec3},
};

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (bool, Color, Ray);
    fn emitted(&self, uv: &(f32, f32), p: &Point3) -> Color {
        // Return black by default.
        Color::from(0.0)
    }
}

#[derive(Clone)]
pub struct Lambertian {
    pub albedo: Rc<dyn Texture>,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self {
            albedo: Rc::new(SolidColor::new(albedo)),
        }
    }

    pub fn from_texture(albedo: Rc<dyn Texture>) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (bool, Color, Ray) {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered = Ray::new(rec.p, scatter_direction, r_in.time);
        let attenuation = self.albedo.value(&rec.uv, rec.p);

        (true, attenuation, scattered)
    }
}

#[derive(Clone, Default)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f32) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (bool, Color, Ray) {
        let reflected = Vec3::reflect(&Vec3::unit_vector(r_in.direction), &rec.normal);

        let scattered = Ray::new(
            rec.p,
            reflected + self.fuzz * Vec3::random_in_unit_sphere(),
            r_in.time,
        );
        let is_scattered = Vec3::dot(&reflected, &rec.normal) > 0.0;

        (is_scattered, self.albedo, scattered)
    }
}

#[derive(Clone, Default)]
pub struct Dielectric {
    pub ir: f32,
}

impl Dielectric {
    pub fn new(ir: f32) -> Self {
        Self { ir }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (bool, Color, Ray) {
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = Vec3::unit_vector(r_in.direction);
        let cos_theta = f32::min(Vec3::dot(&-unit_direction, &rec.normal), 1.0);
        let sin_theta = f32::sqrt(1.0 - cos_theta * cos_theta);

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract || reflectance(cos_theta, refraction_ratio) > random() {
            Vec3::reflect(&unit_direction, &rec.normal)
        } else {
            Vec3::refract(&unit_direction, &rec.normal, refraction_ratio)
        };

        let scattered = Ray::new(rec.p, direction, r_in.time);

        (true, Color::from(1.0), scattered)
    }
}

fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
    // Use Schlick's approximation for reflectance.
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * f32::powi(1.0 - cosine, 5)
}

#[derive(Clone)]
pub struct DiffuseLight {
    emit: Rc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(emission_texture: Rc<dyn Texture>) -> Self {
        Self {
            emit: emission_texture,
        }
    }

    pub fn from_color(color: Color) -> Self {
        Self {
            emit: Rc::new(SolidColor::new(color)),
        }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, r_in: &Ray, _rec: &HitRecord) -> (bool, Color, Ray) {
        (false, Color::from(0.0), r_in.clone())
    }

    fn emitted(&self, uv: &(f32, f32), p: &Point3) -> Color {
        self.emit.value(uv, p.clone())
    }
}
