use std::default::Default;
use crate::tracer::vec3::{self, Vec3};
use crate::tracer::ray::{Ray};
use crate::utils::random_in_unit_disk;
use crate::utils::Random;

pub struct Camera {
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub origin: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub lens_radius: f32,
}

impl Default for Camera {
    fn default() -> Self { 
        Self {
            lower_left_corner: Vec3{e: [-2.0, -1.0, -1.0]},
            horizontal: Vec3{e: [4.0, 0.0, 0.0]},
            vertical: Vec3{e: [0.0, 2.0, 0.0]},
            origin: Vec3{e: [0.0, 0.0, 0.0]},
            u: Vec3{e: [0.0, 0.0, 0.0]},
            v: Vec3{e: [0.0, 0.0, 0.0]},
            w: Vec3{e: [0.0, 0.0, 0.0]},
            lens_radius: 0.0,
        }
    }
}

impl Camera {
    pub fn new(
            lookfrom: Vec3, 
            lookat: Vec3, 
            vup: Vec3, 
            vfov: f32, 
            aspect: f32,
            aperture: f32, 
            focus_dist: f32,
        ) -> Camera {
        let w: Vec3 = (lookfrom - lookat).unit_vector();
        let u: Vec3 = (vec3::cross(vup, w)).unit_vector();
        let v: Vec3 = vec3::cross(w, u);
        let theta: f32 = vfov*std::f32::consts::PI/180 as f32;
        let half_height: f32 = (theta/2.0).tan();
        let half_width: f32 = aspect * half_height;
        Camera {
            lower_left_corner: 
                lookfrom - 
                half_width*focus_dist*u - 
                half_height*focus_dist*v - 
                focus_dist*w,
            horizontal: 2.0 * half_width * u * focus_dist,
            vertical: 2.0 * half_height * v * focus_dist,
            origin: lookfrom,
            lens_radius: aperture/2.0,
            u: u,
            v: v,
            w: w,
        }
        
    }
    pub fn get_ray(&self, u: f32, v: f32, random: &mut Random) -> Ray {
        let rd : Vec3 = self.lens_radius*random_in_unit_disk(random);
        let offset: Vec3 = self.u * rd.x() + self.v * rd.y();
        return Ray{a: self.origin + offset, b: (self.lower_left_corner + u*self.horizontal) + v*self.vertical - self.origin - offset};
    }
}