use std::rc::Rc;
use crate::material::base::Material;
use crate::material::base::ScatterRecord;
use crate::objects::hittable::HitRecord;
use crate::tracer::color::Color3;
use crate::tracer::ray::Ray;
use crate::tracer::vec3::Vec3;
use crate::utils::{self, Random};
use crate::textures::texture::Texture;
use crate::textures::solid_color::SolidColor;

pub struct LambertMaterial {
    pub albedo: Rc<dyn Texture>,
}

impl LambertMaterial {
    pub fn new(albedo: Color3) -> Self {
        LambertMaterial {
            albedo: Rc::new(SolidColor::new(albedo))
        }
    }

    pub fn new_from_texture(albedo: Rc<dyn Texture>) -> Self {
        LambertMaterial {
            albedo
        }
    }
}

impl Material for LambertMaterial {
    fn scatter(&self, mut random: &mut Random, _r_in: Ray, hit_record: HitRecord) -> Option<ScatterRecord> {
        let mut target: Vec3 = (hit_record.normal) + utils::random_in_unit_sphere(&mut random);
        if target.near_zero() {
            target = hit_record.normal;
        }
        Some(ScatterRecord {
            attenuation: self.albedo.value(hit_record.u, hit_record.v, &hit_record.p),
            scattered: Ray{a: hit_record.p, b: target, 
                _time: _r_in.time(),
            }
        })
    }
}