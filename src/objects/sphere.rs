use crate::objects::hittable;
use crate::tracer::vec3 as vec3;
use crate::tracer::ray as ray;

pub trait Hittable  {
    fn hit(&self, r: ray::Ray, t_min: f32, t_max: f32, rec: crate::objects::hittable::HitRecord) -> bool;
}

pub struct Sphere {
    pub center: vec3::Vec3,
    pub radius: f32,
}

impl Hittable for Sphere {
    fn hit(&self, r: ray::Ray, t_min: f32, t_max: f32, rec: hittable::HitRecord) -> bool {
        let oc: vec3::Vec3 = r.origin() - self.center;
        let a: f32 = vec3::dot(r.direction(), r.direction());
        let b: f32 = vec3::dot(oc, r.direction());
        let c: f32 = vec3::dot(oc, oc) - self.radius*self.radius;
        let discriminant = b*b - a*c;
        if discriminant > 0.0 {
            let mut temp: f32 = (-b - discriminant.sqrt())/a;
            if (temp < t_max) & (temp > t_min) {
                rec = hittable::HitRecord{t: temp, p: r.point_at_parameter(rec.t), normal: (rec.p - self.center) / self.radius};
                return true;
            }
            temp = (-b + discriminant.sqrt()) / a;
            if (temp < t_max) & (temp > t_min) {
                rec = hittable::HitRecord{t: temp, p: r.point_at_parameter(rec.t), normal: (rec.p - self.center) / self.radius};
                return true;
            }
        }
        return false;
    }
}

