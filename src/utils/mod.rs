use rand;
use rand::prelude::*;

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

