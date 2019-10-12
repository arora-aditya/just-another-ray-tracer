use rand;
use rand::prelude::*;
use crate::tracer::vec3::Vec3;

pub struct Random {
    seed:  [u8; 32],
    pub rng: rand::rngs::StdRng
}

impl Default for Random {
    fn default() -> Self { 
        Self {
            seed:  [13; 32],
            rng: rand::SeedableRng::from_seed([13; 32]),
        }
    }
}

impl Random {    
    pub fn f32(&mut self) -> f32 {
        return self.rng.gen();
    }
}

pub fn random_in_unit_sphere(random: &mut Random) -> Vec3 {
    let mut p: Vec3 = Default::default();
    while p.squared_length() >= 1.0 {
        p = 2.0*Vec3{e: [random.f32(), random.f32(), random.f32()]} - Vec3{ e: [1.0, 1.0, 1.0] };
    }
    p
}

