use crate::material::base::Material;
use crate::material::base::ScatterRecord;
use crate::objects::hittable::HitRecord;
use crate::tracer::color::Color3;
use crate::tracer::ray::Ray;
use crate::tracer::vec3::{self, Vec3};
use crate::utils::{self, Random};

pub struct MetalMaterial {
    pub albedo: Color3,
    pub f: f32,
}

impl MetalMaterial {
    pub fn fuzz(&self) -> f32 {
        if self.f < 1.0 {self.f} else {1.0}
    }
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - ((n * 2.0) * vec3::dot(v, n))
}

impl Material for MetalMaterial {
    fn scatter(&self, mut random: &mut Random, r_in: Ray, hit_record: HitRecord) -> Option<ScatterRecord> {
        let reflected: Vec3 = reflect(r_in.direction().unit_vector(), hit_record.normal);
        let scattered: Ray  = Ray {
                                a: hit_record.p, 
                                b: reflected + (utils::random_in_unit_sphere(random) * self.fuzz())};
        if vec3::dot(scattered.direction(), hit_record.normal) > 0.0 {
            Some(ScatterRecord {
                attenuation: self.albedo,
                scattered: scattered
            })
        } else {
            None
        }
    }
}