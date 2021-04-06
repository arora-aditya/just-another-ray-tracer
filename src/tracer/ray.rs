use std::fmt;

use crate::tracer::vec3::Vec3;

#[derive(Copy, Clone)]
pub struct Ray {
    pub a: Vec3,
    pub b: Vec3,
    pub _time: f32,
}

impl Ray {
    pub fn new(origin: Vec3, dir: Vec3, time: f32) -> Ray {
       Ray {
           a: origin,
           b: dir,
           _time: time,
       }
   }
    pub fn origin(&self) -> Vec3 {
        self.a
    }
    
    pub fn direction(&self) -> Vec3 {
        self.b
    }
    
    pub fn point_at_parameter(&self, t: f32) -> Vec3 {
        self.a + self.b*t
    }
    
    pub fn time(&self) -> f32 {
        self._time
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
                a: Vec3{e: [1.0, 0.0, 2.0]},
                b: Vec3{e: [4.0, 5.0, 6.0]},
            }.point_at_parameter(4.0),
            Vec3{e: [17.0, 20.0, 26.0]}
        );
    }
    
    #[test]
    fn origin_test() {
        assert_eq!(
            Ray {
                a: Vec3{e: [1.0, 0.0, 2.0]},
                b: Vec3{e: [4.0, 5.0, 6.0]},
            }.origin(),
            Vec3{e: [1.0, 0.0, 2.0]}
        );
    }
    
    #[test]
    fn direction_test() {
        assert_eq!(
            Ray {
                a: Vec3{e: [1.0, 0.0, 2.0]},
                b: Vec3{e: [4.0, 5.0, 6.0]},
            }.direction(),
            Vec3{e: [4.0, 5.0, 6.0]}
        );
    }
}