use crate::tracer::ray::Ray;
use crate::objects::hittable::Hittable;
use crate::objects::hittable::HitRecord;

pub struct HittableList<H: Hittable> {
    pub hitables: std::vec::Vec<H>
}

impl<H: Hittable> Hittable for HittableList<H> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_so_far: f32 = t_max;
        let mut hit_record_opt: Option<HitRecord> = None;
        for hitable in &self.hitables {
            if let Some(hit_record) = hitable.hit(r, t_min, closest_so_far) {
                closest_so_far = hit_record.t;
                hit_record_opt = Some(hit_record);
            }
        }
        hit_record_opt
    }
}