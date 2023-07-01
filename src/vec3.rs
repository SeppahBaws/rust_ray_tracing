use rand::Rng;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};

use crate::utils::random;

#[derive(Clone, Copy, Debug, Default)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub type Point3 = Vec3;
pub type Color = Vec3;

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn from(f: f32) -> Self {
        Self { x: f, y: f, z: f }
    }

    pub fn random() -> Self {
        Self {
            x: random(),
            y: random(),
            z: random(),
        }
    }

    pub fn random_range(min: f32, max: f32) -> Self {
        Self {
            x: rand::thread_rng().gen_range(min..max),
            y: rand::thread_rng().gen_range(min..max),
            z: rand::thread_rng().gen_range(min..max),
        }
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Vec3::random_range(-1.0, 1.0);
            if p.length_squared() >= 1.0 {
                continue;
            }
            return p;
        }
    }

    pub fn random_unit_vector() -> Self {
        Vec3::unit_vector(Vec3::random_in_unit_sphere())
    }

    pub fn random_in_unit_disk() -> Self {
        loop {
            let p = Vec3::new(
                rand::thread_rng().gen_range(-1.0..1.0),
                rand::thread_rng().gen_range(-1.0..1.0),
                0.0,
            );
            if p.length_squared() >= 1.0 {
                continue;
            }
            return p;
        }
    }

    pub fn random_in_hemisphere(normal: &Vec3) -> Self {
        let in_unit_sphere = Vec3::random_in_unit_sphere();
        if Vec3::dot(&in_unit_sphere, &normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    pub fn length(&self) -> f32 {
        f32::sqrt(self.length_squared())
    }

    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        f32::abs(self.x) < s && f32::abs(self.y) < s && f32::abs(self.z) < s
    }

    pub fn dot(u: &Self, v: &Self) -> f32 {
        u.x * v.x + u.y * v.y + u.z * v.z
    }

    pub fn cross(u: &Self, v: &Self) -> Self {
        Self {
            x: u.y * v.z - u.z * v.y,
            y: u.z * v.x - u.x * v.z,
            z: u.x * v.y - u.y * v.x,
        }
    }

    pub fn unit_vector(v: Self) -> Self {
        v / v.length()
    }

    pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
        *v - 2.0 * Vec3::dot(v, n) * *n
    }

    pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f32) -> Vec3 {
        let cos_theta = f32::min(Vec3::dot(&-*uv, n), 1.0);
        let r_out_perp = etai_over_etat * (*uv + cos_theta * *n);
        let r_out_parallel = -f32::sqrt(f32::abs(1.0 - r_out_perp.length_squared())) * *n;

        r_out_perp + r_out_parallel
    }
}

// -vec
impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

// vec += vec
impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        };
    }
}

// vec *= float
impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, t: f32) {
        *self = Self {
            x: self.x * t,
            y: self.y * t,
            z: self.z * t,
        };
    }
}

// vec /= f32
impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, t: f32) {
        *self *= 1.0 / t;
    }
}

// vec + vec
impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

// vec - vec
impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

// vec * vec
impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

// vec * f32
impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, t: f32) -> Self {
        Self {
            x: self.x * t,
            y: self.y * t,
            z: self.z * t,
        }
    }
}

// f32 * vec
impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

// vec / f32
impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, t: f32) -> Self {
        (1.0 / t) * self
    }
}
