use crate::tracer::ray;
use crate::tracer::vec3::Vec3;
use crate::objects::hittable;

pub struct HittableList {
    list: std::vec::Vec<Box<dyn hittable::Hittable>>
}

impl hittable::Hittable for HittableList {
    fn hit(&self, r: ray::Ray, t_min: f32, t_max: f32, mut rec: hittable::HitRecord) -> hittable::HitRecord {
        let mut temp_rec = hittable::HitRecord {
                t: 0.0,
                p: Vec3{e: [0.0,0.0,0.0]},
                normal: Vec3{e: [0.0,0.0,0.0]},
                hit: false,
            };
        let mut closest_so_far: f32 = t_max;
        for hittable_ in self.list.iter() {
            temp_rec = hittable_.hit(r, t_min, closest_so_far, temp_rec);
            if temp_rec.hit {
                closest_so_far = temp_rec.t;
                rec = temp_rec;
            }
        }
        rec
    }
}