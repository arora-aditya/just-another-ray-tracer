use std::default::Default;
use std::ops::Index;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Sub;
use std::ops::SubAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Neg;
use std::ops::Div;
use std::ops::DivAssign;
use std::fmt;
use crate::utils::{self, Random};


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec3 {
    pub e: [f32; 3],
}

impl Default for Vec3 {
    fn default() -> Self { 
        Self {
            e: [1.0, 1.0, 1.0],
        }
    }
}

pub enum RandomVectorType {
    InUnitDisk,
    InUnitSphere,
    Unit,
}

impl Vec3 {
    pub const TOL: f64 = 1e-8;

    pub fn x(&self) -> f32 {
        self.e[0]
    }
    pub fn y(&self) -> f32 {
        self.e[1]
    }
    pub fn z(&self) -> f32 {
        self.e[2]
    }
    
    pub fn r(&self) -> f32 {
        self.e[0]
    }
    pub fn g(&self) -> f32 {
        self.e[1]
    }
    pub fn b(&self) -> f32 {
        self.e[2]
    }

    pub fn neg(&self) -> Vec3 {
        return Vec3 { e: [-1.0 * self.e[0], -1.0 * self.e[1], -1.0 * self.e[2]] };
    }
    
    pub fn random(vector_type: RandomVectorType) -> Vec3 {
        let mut random: utils::Random = Default::default();
        match vector_type {
            RandomVectorType::InUnitDisk => utils::random_in_unit_disk(&mut random),
            RandomVectorType::InUnitSphere => utils::random_in_unit_sphere(&mut random),
            RandomVectorType::Unit => utils::random_unit_vector(&mut random),
        }
    }
    
    pub fn near_zero(&self) -> bool {
        f64::abs(self.x().into()) < Self::TOL && f64::abs(self.y().into()) < Self::TOL && f64::abs(self.z().into()) < Self::TOL
    }
    
    pub fn make_unit_vector(&mut self){
        let k: f32 = (self.e[0]*self.e[0] + self.e[1]*self.e[1] + self.e[2]*self.e[2]).sqrt();
        println!("{}", k);
        *self = Self {
            e: [self.e[0]/k, self.e[1]/k, self.e[2]/k],
        };
    }
    
    pub fn length(self) -> f32 {
        (self.e[0]*self.e[0] + self.e[1]*self.e[1] + self.e[2]*self.e[2]).sqrt()
    }
    
    pub fn squared_length(self) -> f32 {
        (self.e[0]*self.e[0] + self.e[1]*self.e[1] + self.e[2]*self.e[2])
    }
    
    pub fn unit_vector(self) -> Vec3 {
        self/self.length()
    }
}

pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3{e: [x as f32, y as f32, z as f32]}
}

pub fn dot(v1: Vec3, v2: Vec3) -> f32 {
    return v1.e[0]*v2.e[0]
         + v1.e[1]*v2.e[1]
         + v1.e[2]*v2.e[2];
}

pub fn cross(v1: Vec3, v2: Vec3) -> Vec3 {
     Vec3
        {   
            e: [
                v1.e[1] * v2.e[2] - v1.e[2] * v2.e[1],
                v1.e[2] * v2.e[0] - v1.e[0] * v2.e[2],
                v1.e[0] * v2.e[1] - v1.e[1] * v2.e[0]
            ]
        }
}

impl fmt::Display for Vec3 {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}

impl Index<u32> for Vec3 {
    type Output = f32;
    fn index(&self, i: u32) -> &f32 {
        &self.e[i as usize]
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        -1.0 * self
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self{
        Self {
            e: [
                self.e[0] + other.e[0], 
                self.e[1] + other.e[1], 
                self.e[2] + other.e[2]
            ],
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            e: [self.e[0] + other.e[0], self.e[1] + other.e[1], self.e[2] + other.e[2]],
        };
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            e: [self.e[0] - other.e[0], self.e[1] - other.e[1], self.e[2] - other.e[2]],
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            e: [self.e[0] - other.e[0], self.e[1] - other.e[1], self.e[2] - other.e[2]],
        };
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, t: f32) -> Self {
        Self{
            e: [self.e[0] * t, self.e[1] * t, self.e[2] * t]
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, t: Vec3) -> Vec3 {
        Vec3{
            e: [self * t.e[0], self * t.e[1], self * t.e[2]]
        }
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, t: f32) {
        self.e = [self.e[0] * t, self.e[1] * t, self.e[2] * t]
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;
    fn mul(self, other: Vec3) -> Vec3 {
        Self{
            e: [self.e[0] * other.e[0], self.e[1] * other.e[1], self.e[2] * other.e[2]]
        }
    }
}

impl MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, other: Vec3) {
        self.e = [self.e[0] * other.e[0], self.e[1] * other.e[1], self.e[2] * other.e[2]]
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Self;
    fn div(self, other: Vec3) -> Vec3 {
        Self{
            e: [self.e[0] / other.e[0], self.e[1] / other.e[1], self.e[2] / other.e[2]]
        }
    }
}

impl DivAssign<Vec3> for Vec3 {
    fn div_assign(&mut self, other: Vec3) {
        self.e = [self.e[0] / other.e[0], self.e[1] / other.e[1], self.e[2] / other.e[2]]
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;
    fn div(self, t: f32) -> Vec3 {
        Self{
            e: [self.e[0] / t, self.e[1] / t, self.e[2] / t]
        }
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, t: f32) {
        self.e = [self.e[0] / t, self.e[1] / t, self.e[2] / t]
    }
}

#[cfg(test)]
mod test {
    use super::Vec3;

    #[test]
    fn add() {
        assert_eq!(
            Vec3 {
                e: [1.0, 0.0, 2.0],
            } + Vec3 {
                e: [2.0, 1.0, 2.0]
            },
            Vec3 {
                e: [3.0, 1.0, 4.0]
            }
        );
    }

    #[test]
    fn cross_test() {
        assert_eq!(
            super::cross(Vec3 {
                e: [2.0, 1.0, 2.0]
            },
            Vec3 {
                e: [1.0, 0.0, 2.0]
            }),
            Vec3 {
                e: [2.0, -2.0, -1.0]
            }
        );
    }

    #[test]
    fn dot_test() {
        assert_eq!(
            super::dot(Vec3 {
                e: [2.0, 1.0, 2.0]
            },
            Vec3 {
                e: [1.0, 0.0, 2.0],
            }),
            6.0
        );
    }

    #[test]
    fn length() {
        let v = Vec3 {
            e: [-2.0, -2.0, 1.0]
        };
        let u = Vec3 {
            e: [0.0, 1.0, 0.0]
        };
        assert_eq!(v.length(), 3.0);
        assert_eq!(u.length(), 1.0);
    }

    #[test]
    fn squared_length() {
        let v = Vec3 {
            e: [-2.0, -2.0, 1.0]
        };
        let u = Vec3 {
            e: [0.0, 1.0, 0.0]
        };
        assert_eq!(v.squared_length(), 9.0);
        assert_eq!(u.squared_length(), 1.0);
    }

    #[test]
    fn mul() {
        assert_eq!(
            3.0 * Vec3 {
                e: [1.0, 2.0, 3.0]
            },
            Vec3 {
                e: [3.0, 6.0, 9.0]
            }
        );
    }

    #[test]
    fn neg() {
        assert_eq!(
            -Vec3 {
                e: [1.0, -2.0, 3.0]
            },
            Vec3 {
                e: [-1.0, 2.0, -3.0]
            }
        );
    }

    #[test]
    fn sub() {
        assert_eq!(
            Vec3 {
                e: [1.0, 0.0, 2.0]
            } - Vec3 {
                e: [2.0, 1.0, 2.0]
            },
            Vec3 {
                e: [-1.0, -1.0, 0.0]
            }
        );
    }
}