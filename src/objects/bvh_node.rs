use std::cmp::Ordering;
use std::rc::Rc;

use crate::objects::aabb::{AABB, surrounding_box};
use crate::objects::hittable::{HitRecord, Hittable};
use crate::tracer::vec3;
use crate::utils::random_usize_in_range;
use crate::tracer::ray::Ray;

pub struct BvhNode {
    pub left: Rc<dyn Hittable>,
    pub right: Rc<dyn Hittable>,
    pub bounding_box: AABB,
}

impl BvhNode {
    pub fn new(
        src_objects: &mut Vec<Rc<dyn Hittable>>,
        start: usize,
        end: usize,
        time0: f32,
        time1: f32,
    ) -> BvhNode {
        let left;
        let right;
        let mut objects = src_objects.clone();
        let axis = random_usize_in_range(0, 3);
        let comparator = match axis {
            0 => BvhNode::box_x_compare,
            1 => BvhNode::box_y_compare,
            2 => BvhNode::box_z_compare,
            _ => panic!("Undefined axis: {}", axis)
        };

        let object_span = end - start;
        match object_span {
            1 => {
                left = objects[start].clone();
                right = objects[start].clone();
            }
            2 => {
                if comparator(&objects[start], &objects[start + 1]) == Ordering::Less {
                    left = objects[start].clone();
                    right = objects[start + 1].clone();
                } else {
                    left = objects[start + 1].clone();
                    right = objects[start].clone();
                }
            }
            _ => {
                let mid = start + object_span / 2;

                objects.sort_by(comparator);
                left = Rc::new(BvhNode::new(&mut objects, start, mid, time0, time1));
                right = Rc::new(BvhNode::new(&mut objects, mid, end, time0, time1));
            }
        }

        let lbbox = left
            .bounding_box(time0, time1)
            .expect("No bounding box in BVHNode constructor");
        let rbbox = right
            .bounding_box(time0, time1)
            .expect("No bounding box in BVHNode constructor");
        return BvhNode {
            left: left.clone(),
            right: right.clone(),
            bounding_box: surrounding_box(&lbbox, &rbbox),
        };
    }
    
    pub fn box_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>, axis: u32) -> std::cmp::Ordering {
        let box_a = a
            .bounding_box(0.0, 0.0)
            .expect("No bounding box in BVHNode constructor");
        let box_b = b
            .bounding_box(0.0, 0.0)
            .expect("No bounding box in BVHNode constructor");

        box_a.minimum[axis]
            .partial_cmp(&box_b.minimum[axis])
            .unwrap()
    }
    
    pub fn box_x_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> std::cmp::Ordering {
        BvhNode::box_compare(a, b, 0)
    }

    pub fn box_y_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> std::cmp::Ordering {
        BvhNode::box_compare(a, b, 1)
    }

    pub fn box_z_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> std::cmp::Ordering {
        BvhNode::box_compare(a, b, 2)
    }
}



impl Hittable for BvhNode {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if !self.bounding_box.hit(ray, t_min, t_max) {
            return None;
        }

        match self.left.hit(ray, t_min, t_max) {
            Some(lrec) => match self.right.hit(ray, t_min, lrec.t) {
                Some(rrec) => return Some(rrec),
                None => return Some(lrec),
            },
            None => return self.right.hit(ray, t_min, t_max),
        }
    }

    fn bounding_box(&self, time0: f32, time1: f32) -> Option<AABB> {
        Some(self.bounding_box)
    }
}