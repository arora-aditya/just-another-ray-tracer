use crate::material::base::Material;
use crate::material::base::ScatterRecord;
use crate::objects::hittable::HitRecord;
use crate::tracer::color::Color3;
use crate::tracer::ray::Ray;
use crate::tracer::vec3::{self, Vec3};
use crate::utils::{self, Random};

pub struct DielectricMaterial {
    pub ref_idx: f32,
}

fn refract(v: Vec3, n: Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let uv: Vec3 = v.unit_vector();
    let dt: f32 = vec3::dot(uv, n);
    let discriminant: f32 = 1.0 - ni_over_nt*ni_over_nt*(1.0-dt*dt);
    if discriminant > 0.0 {
        Some(
            ((uv - (n * dt)) * ni_over_nt) - (n * discriminant.sqrt())
        )
    }
    else {
        None
    }
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - ((n * 2.0) * vec3::dot(v, n))
}

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let mut r0: f32 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0*r0;
    r0 + (1.0-r0)*(1.0 - cosine).powf(5.0)
}

impl Material for DielectricMaterial {
    fn scatter(&self, mut random: &mut Random, r_in: Ray, hit_record: HitRecord) -> Option<ScatterRecord> {        
        let attenuation: Color3 = Color3 {r: 1.0, g: 1.0, b: 1.0};
        let reflected: Vec3 = reflect(r_in.direction(), hit_record.normal);
        let (outward_normal, ni_over_nt, cosine) =
            if vec3::dot(r_in.direction(), hit_record.normal) > 0.0 {
                let cosine: f32 = self.ref_idx * vec3::dot(hit_record.normal, r_in.direction()) / r_in.direction().length();
                (
                    -1.0 * hit_record.normal, 
                    self.ref_idx,
                    cosine,
                )
            } else {
                let cosine: f32 = -vec3::dot(r_in.direction(), hit_record.normal) / r_in.direction().length();
                (
                    hit_record.normal, 
                    1.0 / self.ref_idx,
                    cosine,
                )
            };
            let reflect_prob: f32 = schlick(cosine, self.ref_idx);

            let r: f32 = random.f32();
            match refract(r_in.direction(), outward_normal, ni_over_nt) {
                Some(refracted) if reflect_prob <= r => {
                    Some(ScatterRecord {
                        attenuation,
                        scattered: Ray{
                            a: hit_record.p, 
                            b: refracted,
                            _time: r_in.time(),
                        }
                    })
                },
                _ => {
                   Some(ScatterRecord {
                        attenuation,
                        scattered: Ray{
                            a: hit_record.p, 
                            b: reflected,
                            _time: r_in.time(),
                        }
                    })
                }
            }
    }
}