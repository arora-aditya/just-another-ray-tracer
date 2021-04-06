use rand;
use rand::prelude::*;
use crate::tracer::vec3::{self, Vec3};

pub struct Random {
    seed:  [u8; 32],
    pub rng: rand::rngs::ThreadRng,
}

impl Default for Random {
    fn default() -> Self { 
        Self {
            seed: [13; 32],
            rng: rand::thread_rng(),
        }
    }
}

impl Random {    
    pub fn f32(&mut self) -> f32 {
        return self.rng.gen::<f32>();
    }
}

pub fn random_in_unit_sphere(random: &mut Random) -> Vec3 {
    let mut p: Vec3 = Default::default();
    while p.squared_length() >= 1.0 {
        p = 2.0*Vec3{e: [random.f32(), random.f32(), random.f32()]} - Vec3{ e: [1.0, 1.0, 1.0] };
    }
    p
}

pub fn random_in_unit_disk(random: &mut Random) -> Vec3 {
    let mut p: Vec3 = Default::default();
    while vec3::dot(p, p) >= 1.0 {
        p = 2.0*Vec3{e: [random.f32(), random.f32(), 0.0]} - Vec3{ e: [1.0, 1.0, 0.0] };
    }
    p
}

pub fn random_usize_in_range(min: usize, max: usize) -> usize {
    rand::thread_rng().gen_range(min, max)
}

pub fn random_unit_vector(random: &mut Random) -> Vec3 {
    random_in_unit_sphere(random).unit_vector()
}

