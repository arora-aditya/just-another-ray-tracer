use std::fmt;

use crate::tracer::vec3;

#[derive(Copy, Clone)]
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

#[cfg(test)]
mod test {
    use super::vec3;
    use super::Ray;

    #[test]
    fn point_at_parameter_test() {
        assert_eq!(
            Ray {
                a: vec3::Vec3{e: [1.0, 0.0, 2.0]},
                b: vec3::Vec3{e: [4.0, 5.0, 6.0]},
            }.point_at_parameter(4.0),
            vec3::Vec3{e: [17.0, 20.0, 26.0]}
        );
    }
    
    #[test]
    fn origin_test() {
        assert_eq!(
            Ray {
                a: vec3::Vec3{e: [1.0, 0.0, 2.0]},
                b: vec3::Vec3{e: [4.0, 5.0, 6.0]},
            }.origin(),
            vec3::Vec3{e: [1.0, 0.0, 2.0]}
        );
    }
    
    #[test]
    fn direction_test() {
        assert_eq!(
            Ray {
                a: vec3::Vec3{e: [1.0, 0.0, 2.0]},
                b: vec3::Vec3{e: [4.0, 5.0, 6.0]},
            }.direction(),
            vec3::Vec3{e: [4.0, 5.0, 6.0]}
        );
    }
}