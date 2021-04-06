use crate::tracer::vec3::{self, Vec3, RandomVectorType};
use crate::utils::random_usize_in_range;

const POINT_COUNT: usize = 256;

pub struct Perlin {
    ranvec: [Vec3; POINT_COUNT],

    perm_x: [i32; POINT_COUNT],
    perm_y: [i32; POINT_COUNT],
    perm_z: [i32; POINT_COUNT],
}

impl Perlin {
    pub fn new() -> Self {
        let mut ranvec: [Vec3; 256] = [vec3::new(0.0, 0.0, 0.0); 256];
        for i in 0..ranvec.len() {
            ranvec[i] = Vec3::random(RandomVectorType::Unit);
        }

        Perlin {
            ranvec,

            perm_x: perlin_generate_perm(),
            perm_y: perlin_generate_perm(),
            perm_z: perlin_generate_perm(),
        }
    }

    pub fn noise(self: &Self, p: &Vec3) -> f32 {
        let u = p.x() - f32::floor(p.x());
        let v = p.y() - f32::floor(p.y());
        let w = p.z() - f32::floor(p.z());

        let i = f32::floor(p.x()) as i32;
        let j = f32::floor(p.y()) as i32;
        let k = f32::floor(p.z()) as i32;

        let mut c = [[[vec3::new(0.0, 0.0, 0.0); 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let i = ((i + di) & 255) as usize;
                    let j = ((j + dj) & 255) as usize;
                    let k = ((k + dk) & 255) as usize;

                    let hash = (self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]) as usize;

                    c[di as usize][dj as usize][dk as usize] = self.ranvec[hash];
                }
            }
        }

        perlin_interp(c, u, v, w)
    }

    pub fn turb(self: &Self, p: &Vec3, depth: Option<i32>) -> f32 {
        let depth = depth.unwrap_or(7);

        let mut accum = 0.0;
        let mut temp_p = *p;
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }

        f32::abs(accum)
    }
}

fn perlin_interp(c: [[[Vec3; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
    let uu = u * u * (3.0 - 2.0 * u);
    let vv = v * v * (3.0 - 2.0 * v);
    let ww = w * w * (3.0 - 2.0 * w);

    let mut accum = 0.0;
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let weight_v = vec3::new(u - i as f32, v - j as f32, w - k as f32);
                accum += 1.0
                    * (i as f32 * uu + (1.0 - i as f32) * (1.0 - uu))
                    * (j as f32 * vv + (1.0 - j as f32) * (1.0 - vv))
                    * (k as f32 * ww + (1.0 - k as f32) * (1.0 - ww))
                    * vec3::dot(c[i][j][k], weight_v);
            }
        }
    }
    accum
}

fn perlin_generate_perm() -> [i32; 256] {
    let mut p: [i32; 256] = [0; 256];
    for i in 0..p.len() {
        p[i] = i as i32;
    }

    permute(&mut p, POINT_COUNT);

    p
}

fn permute(p: &mut [i32; POINT_COUNT], n: usize) {
    for i in (1..n - 1).rev() {
        let target = random_usize_in_range(0, i);
        let tmp = p[i];
        p[i] = p[target];
        p[target] = tmp;
    }
}