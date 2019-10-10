use std::ops::Index;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Sub;
use std::ops::SubAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::fmt;


#[derive(Copy, Clone)]
pub struct Vec3 {
    pub e: [f32; 3],
}

impl Vec3 {
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