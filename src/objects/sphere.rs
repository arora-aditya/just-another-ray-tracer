use core::borrow::Borrow;
use libm::{acos, atan2};
use std::f64::consts::PI;
use crate::objects::aabb::{AABB, surrounding_box};
use crate::objects::hittable::HitRecord;
use crate::objects::hittable::Hittable;
use crate::tracer::vec3::{self, Vec3};
use crate::tracer::ray::Ray;
use crate::material::base::Material;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Box<Material>,
    bbox: AABB,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Box<Material>) -> Sphere{
        let bbox = AABB {
            minimum: center - vec3::new(radius, radius, radius),
            maximum: center + vec3::new(radius, radius, radius),
        };
        Sphere {
            center,
            radius,
            material,
            bbox,
        }
    }
    
    pub fn get_sphere_uv(&self, v: Vec3) -> (f32, f32){
        let theta: f64 = acos((-v.y()).into());
        let phi: f64 = atan2((-v.z()).into(), v.x().into()) + PI;

        ((phi / (2.0 * PI)) as f32, (theta / PI) as f32)
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc: Vec3 = r.origin() - self.center;
        let a : f32  = vec3::dot(r.direction(), r.direction());
        let b : f32  = vec3::dot(oc, r.direction());
        let c : f32  = vec3::dot(oc, oc) - self.radius*self.radius;
        let discriminant: f32 = b * b - a * c;

        let temp1: f32  = (-b - discriminant.sqrt()) / a;
        let temp2: f32  = (-b + discriminant.sqrt()) / a;
        let b1   : bool = t_min < temp1 && temp1 < t_max;
        let b2   : bool = t_min < temp2 && temp2 < t_max;
        
        if discriminant > 0.0 && (b1 || b2) {
            let t: f32  = if b1 { temp1 } else {temp2};
            let p: Vec3 = r.point_at_parameter(t);
            let normal: Vec3 = (p - self.center) / self.radius;
            let (u, v): (f32, f32) = self.get_sphere_uv(normal);
            Some(HitRecord{ t, p, normal, material: self.material.borrow(), u, v })
        } else {
            None
        }
    }
    
    fn bounding_box(&self, time0: f32, time1: f32) -> Option<AABB> {
        Some(self.bbox)
    }
    
}

pub struct MovingSphere {
    pub center0: Vec3,
    pub center1: Vec3,
    pub radius: f32,
    pub material: Box<Material>,
    pub time0: f32,
    pub time1: f32,
}

impl MovingSphere {
    pub fn center(&self, time: f32) -> Vec3 {
        self.center0 + ((time - self.time0) / (self.time1 - self.time0))*(self.center1 - self.center0)
    }
    
    pub fn get_sphere_uv(&self, v: Vec3) -> (f32, f32){
        let theta: f64 = acos((-v.y()).into());
        let phi: f64 = atan2((-v.z()).into(), v.x().into()) + PI;

        ((phi / (2.0 * PI)) as f32, (theta / PI) as f32)
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc: Vec3 = r.origin() - self.center(r.time());
        let a : f32  = vec3::dot(r.direction(), r.direction());
        let b : f32  = vec3::dot(oc, r.direction());
        let c : f32  = vec3::dot(oc, oc) - self.radius*self.radius;
        let discriminant: f32 = b * b - a * c;

        let temp1: f32  = (-b - discriminant.sqrt()) / a;
        let temp2: f32  = (-b + discriminant.sqrt()) / a;
        let b1   : bool = t_min < temp1 && temp1 < t_max;
        let b2   : bool = t_min < temp2 && temp2 < t_max;
        if discriminant > 0.0 && (b1 || b2) {
            let t: f32  = if b1 { temp1 } else {temp2};
            let p: Vec3 = r.point_at_parameter(t);
            let normal: Vec3 = (p - self.center(r.time())) / self.radius;
            let (u, v): (f32, f32) = self.get_sphere_uv(normal);
            Some(HitRecord{ t, p, normal, material: self.material.borrow(), u, v })
        } else {
            None
        }
    }
    
    fn bounding_box(&self, time0: f32, time1: f32) -> Option<AABB> {
        let box0: AABB = AABB::new(
            self.center(time0) - vec3::new(self.radius, self.radius, self.radius),
            self.center(time0) + vec3::new(self.radius, self.radius, self.radius)
        );
        let box1: AABB = AABB::new(
            self.center(time1) - vec3::new(self.radius, self.radius, self.radius),
            self.center(time1) + vec3::new(self.radius, self.radius, self.radius));
        return Some(surrounding_box(&box0, &box1))
    }
}

pub enum SphereThing {
    Fixed(Sphere),
    Moving(MovingSphere),
}

impl Hittable for SphereThing {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        match &*self {
            SphereThing::Fixed(s) => s.hit(r, t_min, t_max),
            SphereThing::Moving(m) => m.hit(r, t_min, t_max)
        }
    }

    fn bounding_box(&self, time0: f32, time1: f32) -> Option<AABB> {
        match &*self {
            SphereThing::Fixed(s) => s.bounding_box(time0, time1),
            SphereThing::Moving(m) => m.bounding_box(time0, time1)
        }
    }
}
