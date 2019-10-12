use std::default::Default;
use crate::tracer::vec3::{self, Vec3};
use crate::tracer::ray::{Ray};

pub struct Camera {
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub origin: Vec3,
}

impl Default for Camera {
    fn default() -> Self { 
        Self {
            lower_left_corner: Vec3{e: [-2.0, -1.0, -1.0]},
            horizontal: Vec3{e: [4.0, 0.0, 0.0]},
            vertical: Vec3{e: [0.0, 2.0, 0.0]},
            origin: Vec3{e: [0.0, 0.0, 0.0]},
        }
    }
}

impl Camera {
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        return Ray{a: self.origin, b: (self.lower_left_corner + u*self.horizontal) + v*self.vertical - self.origin};
    }
}