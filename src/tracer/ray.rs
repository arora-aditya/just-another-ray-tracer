use std::fmt;

#[path = "vec3.rs"]
pub mod vec3;

pub struct Ray {
    pub a: vec3::Vec3,
    pub b: vec3::Vec3,
}

impl Ray {
    pub fn origin(&self) -> vec3::Vec3 {
        self.a
    }
    
    pub fn direction(&self) -> vec3::Vec3 {
        self.b
    }
    
    pub fn point_at_parameter(&self, t: f32) -> vec3::Vec3 {
        self.a + self.b*t
    }
}

impl fmt::Display for Ray {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "a: {} b:{}", self.a, self.b)
    }
}