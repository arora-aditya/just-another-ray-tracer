use crate::tracer::color::Color3;
use crate::tracer::ray::Ray;
use crate::objects::hittable::HitRecord;
use crate::tracer::vec3::{self, Vec3};
use crate::utils::{self, Random};

pub struct ScatterRecord{
    pub attenuation: Color3,
    pub scattered: Ray,
}

pub trait Material {
    fn scatter(&self, mut random: &mut Random, r_in: Ray, hit_record: HitRecord) -> Option<ScatterRecord>;
}
