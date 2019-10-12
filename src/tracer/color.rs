use std::ops::{Add, AddAssign, DivAssign, Mul, MulAssign};

#[derive(Debug, Copy, Clone)]
pub struct Color3 {
    pub r: f32,
    pub g: f32,
    pub b: f32
}

fn g2_color_elem_to_int(f: f32) -> i32 {
    // for gamma 2 encoding
    (255.99f32 * f.sqrt()) as i32
}

fn color_elem_to_int(f: f32) -> i32 {
    (255.99f32 * f) as i32
}

impl Color3 {
    pub fn ir(&self) -> i32 {
        color_elem_to_int(self.r)
    }

    pub fn ig(&self) -> i32 {
        color_elem_to_int(self.g)
    }

    pub fn ib(&self) -> i32 {
        color_elem_to_int(self.b)
    }
    
    pub fn g2_ir(&self) -> i32 {
        // for gamma 2 encoding
        g2_color_elem_to_int(self.r)
    }

    pub fn g2_ig(&self) -> i32 {
        // for gamma 2 encoding
        g2_color_elem_to_int(self.g)
    }

    pub fn g2_ib(&self) -> i32 {
        // for gamma 2 encoding
        g2_color_elem_to_int(self.b)
    }
}

impl Add for &Color3 {
    type Output = Color3;

    fn add(self, rhs: Self) -> Self::Output {
        Color3{r: self.r + rhs.r, g: self.g + rhs.g, b: self.b + rhs.b}
    }
}

impl AddAssign for Color3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            r: self.r + other.r,
            g: self.g + other.g, 
            b: self.b + other.b,
        };
    }
}

impl DivAssign<i32> for Color3 {
    fn div_assign(&mut self, other: i32) {
        *self = Color3 {
            r: self.r/other as f32,
            g: self.g/other as f32, 
            b: self.b/other as f32,
        };
    }
}

impl DivAssign<f32> for Color3 {
    fn div_assign(&mut self, other: f32) {
        *self = Color3 {
            r: self.r/other,
            g: self.g/other, 
            b: self.b/other,
        };
    }
}

impl Mul<f32> for Color3 {
    type Output = Color3;

    fn mul(self, rhs: f32) -> Self::Output {
        Color3{r: self.r * rhs, g: self.g * rhs, b: self.b * rhs}
    }
}

impl Mul<f32> for &Color3 {
    type Output = Color3;

    fn mul(self, rhs: f32) -> Self::Output {
        Color3{r: self.r * rhs, g: self.g * rhs, b: self.b * rhs}
    }
}
