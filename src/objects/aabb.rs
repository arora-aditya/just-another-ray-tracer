use libm;
use std::mem;
use core::borrow::Borrow;

use crate::tracer::vec3::{self, Vec3};
use crate::tracer::ray::Ray;
use crate::objects::hittable::Hittable;
use crate::objects::hittable::HitRecord;
use crate::material::metal::MetalMaterial;

#[derive(Copy, Clone)]
pub struct AABB {
    pub minimum: Vec3,
    pub maximum: Vec3,
}

impl AABB {
    pub fn min(&self) -> Vec3 {
        self.minimum
    }
    pub fn max(&self) -> Vec3 {
        self.maximum
    }
}

impl AABB {
    pub fn new(minimum: Vec3, maximum: Vec3) -> AABB {
        AABB{
            minimum: minimum,
            maximum: maximum,
        }
    }
    
    pub fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> bool {
        for a in 0..3 {
            let inv_d: f32 = 1.0 / r.direction()[a];
            let mut t0: f32 = (self.min()[a] - r.origin()[a]) * inv_d;
            let mut t1: f32 = (self.max()[a] - r.origin()[a]) * inv_d;
            if (inv_d < 0.0){
                mem::swap(&mut t0, &mut t1);
            }
            let t_min2 = if (t0 > t_min) { t0 } else { t_min };
            let t_max2 = if (t1 < t_max) { t1 } else { t_max };
            if (t_max2 <= t_min2){
                return false;
            }
        }
        true
        // for a in 0..3 {
        //     let t0: f32 = fmin((minimum[a] - r.origin()[a]) / r.direction()[a],
        //                    (maximum[a] - r.origin()[a]) / r.direction()[a]);
        //     let t1: f32 = fmax((minimum[a] - r.origin()[a]) / r.direction()[a],
        //                    (maximum[a] - r.origin()[a]) / r.direction()[a]);
        //     t_min = fmax(t0, t_min);
        //     t_max = fmin(t1, t_max);
        //     if (t_max <= t_min)
        //         return false;
        // }
        // return true;
    }
}


pub fn surrounding_box(box0: &AABB, box1: &AABB) -> AABB {
    let small: Vec3 = vec3::new(libm::fmin(box0.min().x().into(), box1.min().x().into()) as f32,
                 libm::fmin(box0.min().y().into(), box1.min().y().into()) as f32,
                 libm::fmin(box0.min().z().into(), box1.min().z().into()) as f32);

    let big: Vec3 = vec3::new(libm::fmax(box0.max().x().into(), box1.max().x().into()) as f32,
               libm::fmax(box0.max().y().into(), box1.max().y().into()) as f32,
               libm::fmax(box0.max().z().into(), box1.max().z().into()) as f32);

    return AABB {minimum: small, maximum: big};
}