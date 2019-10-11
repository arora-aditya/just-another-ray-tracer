use crate::tracer::vec3::Vec3;
use crate::tracer::ray as ray;

#[derive(Copy, Clone)]
pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
}

impl HitRecord {
    pub fn zero(&self) {
        self.t = 0.0;
        self.p = Vec3{e: [0.0,0.0,0.0]};
        self.normal = Vec3{e: [0.0,0.0,0.0]};
    }
}

pub trait Hittable  {
    fn hit(&self, r: ray::Ray, t_min: f32, t_max: f32, rec: crate::objects::hittable::HitRecord) -> bool;
}