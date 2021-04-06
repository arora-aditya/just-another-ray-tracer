use std::rc::Rc;
use core::borrow::Borrow;
use crate::tracer::vec3 as vec3;
use crate::tracer::ray::Ray;
use crate::objects::aabb::{AABB, surrounding_box};
use crate::objects::hittable::Hittable;
use crate::objects::hittable::HitRecord;

pub struct HittableList {
    pub hitables: Vec<Rc<dyn Hittable>>,
}

impl Hittable for HittableList {
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
    
    fn bounding_box(&self, time0: f32, time1: f32) -> Option<AABB> {
        if (self.hitables.is_empty()){
            return None;
        }
        let mut output_box: AABB = AABB::new(vec3::new(0.0, 0.0, 0.0), vec3::new(0.0, 0.0, 0.0));
        let mut first_box: bool = true;
        for hitable in &self.hitables {
            if hitable.bounding_box(time0, time1).is_some() {
                let obj_bbox = hitable.bounding_box(time0, time1).unwrap();
                output_box = surrounding_box(&output_box, &obj_bbox)
            }
        }
        return Some(output_box);
    }
}