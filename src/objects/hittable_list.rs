use crate::tracer::ray;
use crate::objects::hittable;

struct HittableList {
    list: std::vec::Vec<Box<dyn hittable::Hittable>>
}

impl hittable::Hittable for HittableList {
    fn hit(&self, r: ray::Ray, t_min: f32, t_max: f32, rec: hittable::HitRecord) -> bool {
        let mut temp_rec: hittable::HitRecord;
        temp_rec.zero();
        let mut hit_anything: bool = false;
        let mut closest_so_far: f32 = t_max;
        for hittable_ in self.list {
            if hittable_.hit(r, t_min, closest_so_far, temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                rec = temp_rec;
            }
        }
        hit_anything
    }
}