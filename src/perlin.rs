use rand::Rng;

use crate::vec3::{Point3, Vec3};

const POINT_COUNT: usize = 256;

pub struct Perlin {
    ranvec: Vec<Vec3>,
    perm_x: Vec<i32>,
    perm_y: Vec<i32>,
    perm_z: Vec<i32>,
}

impl Perlin {
    pub fn new() -> Self {
        let mut ranvec: Vec<Vec3> = Vec::with_capacity(POINT_COUNT);
        for i in 0..POINT_COUNT {
            ranvec.push(Vec3::random_in_unit_sphere());
        }

        let perm_x = generate_perm();
        let perm_y = generate_perm();
        let perm_z = generate_perm();

        Self {
            ranvec,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, p: &Point3) -> f32 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();

        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;

        let mut c: [[[Vec3; 2]; 2]; 2] = [
            [
                [Vec3::from(0.0), Vec3::from(0.0)],
                [Vec3::from(0.0), Vec3::from(0.0)],
            ],
            [
                [Vec3::from(0.0), Vec3::from(0.0)],
                [Vec3::from(0.0), Vec3::from(0.0)],
            ],
        ];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let idx_x = ((i + di) & 255) as usize;
                    let idx_y = ((j + dj) & 255) as usize;
                    let idx_z = ((k + dk) & 255) as usize;

                    c[di as usize][dj as usize][dk as usize] = self.ranvec
                        [(self.perm_x[idx_x] ^ self.perm_y[idx_y] ^ self.perm_z[idx_z]) as usize];
                }
            }
        }

        perlin_interp(&c, u, v, w)
    }

    pub fn turb(&self, p: &Point3) -> f32 {
        self.turb_scale(&p, 7)
    }

    pub fn turb_scale(&self, p: &Point3, depth: i32) -> f32 {
        let mut accum = 0.0;
        let mut temp_p = *p;
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }

        accum.abs()
    }
}

fn generate_perm() -> Vec<i32> {
    let mut randfloat: Vec<i32> = Vec::with_capacity(POINT_COUNT);
    for i in 0..POINT_COUNT {
        randfloat.push(i as i32);
    }

    permutate(&mut randfloat);

    randfloat
}

fn permutate(p: &mut Vec<i32>) {
    for i in (0..p.len()).rev() {
        let target = rand::thread_rng().gen_range(0..p.len());
        let tmp = p[i];
        p[i] = p[target];
        p[target] = tmp;
    }
}

fn perlin_interp(c: &[[[Vec3; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
    let uu = u * u * (3.0 - 2.0 * u);
    let vv = v * v * (3.0 - 2.0 * v);
    let ww = w * w * (3.0 - 2.0 * w);
    let mut accum = 0.0;

    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let weight_v = Vec3::new(u - (i as f32), v - (j as f32), w - (k as f32));

                let i_f = i as f32;
                let j_f = j as f32;
                let k_f = k as f32;

                accum += (i_f * uu + (1.0 - i_f) * (1.0 - uu))
                    * (j_f * vv + (1.0 - j_f) * (1.0 - vv))
                    * (k_f * ww + (1.0 - k_f) * (1.0 - ww))
                    * Vec3::dot(&c[i][j][k], &weight_v);
            }
        }
    }

    accum
}
