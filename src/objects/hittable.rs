use crate::tracer::vec3::Vec3;
use crate::tracer::ray as ray;

#[derive(Copy, Clone)]
pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub hit: bool,
}

impl HitRecord {
    pub fn zero(&mut self) {
        self.t = 0.0;
        self.p = Vec3{e: [0.0,0.0,0.0]};
        self.normal = Vec3{e: [0.0,0.0,0.0]};
        self.hit = false;
    }
}

pub trait Hittable  {
    fn hit(&self, r: ray::Ray, t_min: f32, t_max: f32, rec: crate::objects::hittable::HitRecord) -> HitRecord;
}