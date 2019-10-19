use crate::material::base::Material;
use crate::material::base::ScatterRecord;
use crate::objects::hittable::HitRecord;
use crate::tracer::color::Color3;
use crate::tracer::ray::Ray;
use crate::tracer::vec3::Vec3;
use crate::utils::{self, Random};

pub struct LambertMaterial {
    pub albedo: Color3,
}

impl Material for LambertMaterial {
    fn scatter(&self, mut random: &mut Random, _r_in: Ray, hit_record: HitRecord) -> Option<ScatterRecord> {
        let target: Vec3 = (hit_record.p + hit_record.normal) + utils::random_in_unit_sphere(&mut random);
        Some(ScatterRecord {
            attenuation: self.albedo,
            scattered: Ray{a: hit_record.p, b: target - hit_record.p, 
                _time: _r_in.time(),
            }
        })
    }
}