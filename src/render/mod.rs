use std::io;
use std::io::Write;

use crate::utils;

use crate::tracer::ray::{self, Ray};
use crate::tracer::vec3::{self, Vec3};
use crate::tracer::color::Color3;
use crate::objects::hittable_list::HittableList;
use crate::objects::hittable;
use crate::objects::sphere::Sphere;
use crate::objects::hittable::Hittable;
use crate::camera::camera::Camera;

pub fn ppm_test(){
    let nx: u32 = 200;
    let ny: u32 = 100;
    
    println!("P3\n{} {}\n255", nx, ny);
    
    let mut j = ny - 1;
    while j >= 0 {
        let mut i = 0;
        while i < nx {
            let r: f32 = i as f32/nx as f32;
            let g: f32 = j as f32/ny as f32;
            let b: f32 = 0.32;
            let ir: u32 = (255.99*r) as u32;
            let ig: u32 = (255.99*g) as u32;
            let ib: u32 = (255.99*b) as u32;
            println!("{} {} {}", ir, ig, ib);
            // std::cout << ir << " " << ig << " " << ib << "\n";
            i += 1;
        }
        j -= 1;
    }
}

pub fn vec_test(){
    let mut v1 = vec3::Vec3 { e: [0.0,1.0,2.0] };
    println!("v1: {}", v1);
    let v2 = vec3::Vec3 { e: [10.0,11.0,12.0] };
    println!("v2: {}", v2);
    println!("{} {} {}", v1.x(), v1.y(), v1.z());
    println!("{} {} {}", -v1.x(), -v1.y(), -v1.z());
    println!("{} {} {}", -v1[0], v1[1], -v1[2]);
    v1+=v1;
    println!("v1+=v1 {}", v1);
    v1*=0.5;
    v1*=2.0;
    println!("v1*=2 {}", v1);
    v1*=0.5;
    v1+=v2;
    println!("v1+=v2 {}", v1);
    v1-=v2;
    v1*=v2;
    println!("v1*=v2 {}", v1);
    v1/=v2;
    v1/=v2;
    println!("v1/=v2 {}", v1);
    v1*=v2;
    println!("v1 x v2 {}", vec3::cross(v1, v2));
    v1.make_unit_vector();
    println!("unit v1 {}", v1);
}

pub fn ray_test(){
    let v1 = vec3::Vec3 { e: [0.0,1.0,2.0] };
    let v2 = vec3::Vec3 { e: [10.0,11.0,12.0] };
    let r1 = ray::Ray {a: v1, b: v2};
    println!("{}", r1);
}

pub fn blue_shader(){
    pub fn color(r1: ray::Ray) -> vec3::Vec3{
        let unit_direction = vec3::Vec3::unit_vector(r1.direction());
        let t: f32 = (unit_direction.y() + 1.0)*0.5;
        return vec3::Vec3{e: [1.0, 1.0, 1.0]}*(1.0-t) + vec3::Vec3{e: [0.5, 0.7, 1.0]}*t;
    }
    
    let nx: i32 = 200;
    let ny: i32 = 100;
    
    println!("P3\n{} {}\n255", nx, ny);
    
    let lower_left_corner = vec3::Vec3 {e: [-2.0, -1.0, -1.0]};
    let horizontal = vec3::Vec3 {e: [4.0, 0.0, 0.0]};
    let vertical = vec3::Vec3 {e: [0.0, 2.0, 0.0]};
    let origin = vec3::Vec3 {e: [0.0, 0.0, 0.0]};
    
    let mut j: i32 = ny - 1;
    while j > 0 {
        let mut i: i32 = 0;
        while i < nx {
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;
            let r = ray::Ray{a: origin, b: lower_left_corner + u*horizontal + v*vertical};
            let col = color(r);
            let ir: u32 = (255.99*col[0]) as u32;
            let ig: u32 = (255.99*col[1]) as u32;
            let ib: u32 = (255.99*col[2]) as u32;
            println!("{} {} {}", ir, ig, ib);
            // std::cout << ir << " " << ig << " " << ib << "\n";
            i += 1;
        }
        j -= 1;
    }
}

pub fn blue_shader_with_sphere(){
    pub fn hit_sphere(center: vec3::Vec3, radius: f32, r: ray::Ray) -> bool {
        let oc: vec3::Vec3 = r.origin() - center;
        let a: f32 = vec3::dot(r.direction(), r.direction());
        let b: f32 = 2.0 * vec3::dot(oc, r.direction());
        let c: f32 = vec3::dot(oc, oc) - radius*radius;
        let discriminant: f32 = b*b - 4.0*a*c;
        return discriminant > 0.0;
    }
    
    pub fn color(r1: ray::Ray) -> vec3::Vec3{
        if hit_sphere(vec3::Vec3{e: [0.0 ,0.0, -1.0]}, 0.5, r1){
            return vec3::Vec3{e: [1.0, 0.0, 0.0]};
        }
        let unit_direction = vec3::Vec3::unit_vector(r1.direction());
        let t: f32 = (unit_direction.y() + 1.0)*0.5;
        return vec3::Vec3{e: [1.0, 1.0, 1.0]}*(1.0-t) + vec3::Vec3{e: [0.5, 0.7, 1.0]}*t;
    }
    
    let nx: i32 = 200;
    let ny: i32 = 100;
    
    println!("P3\n{} {}\n255", nx, ny);
    
    let lower_left_corner = vec3::Vec3 {e: [-2.0, -1.0, -1.0]};
    let horizontal = vec3::Vec3 {e: [4.0, 0.0, 0.0]};
    let vertical = vec3::Vec3 {e: [0.0, 2.0, 0.0]};
    let origin = vec3::Vec3 {e: [0.0, 0.0, 0.0]};
    
    let mut j: i32 = ny - 1;
    while j > 0 {
        let mut i: i32 = 0;
        while i < nx {
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;
            let r = ray::Ray{a: origin, b: lower_left_corner + u*horizontal + v*vertical};
            let col = color(r);
            let ir: u32 = (255.99*col[0]) as u32;
            let ig: u32 = (255.99*col[1]) as u32;
            let ib: u32 = (255.99*col[2]) as u32;
            println!("{} {} {}", ir, ig, ib);
            // std::cout << ir << " " << ig << " " << ib << "\n";
            i += 1;
        }
        j -= 1;
    }
}

pub fn blue_shader_with_sphere_shading(){
    pub fn hit_sphere(center: vec3::Vec3, radius: f32, r: ray::Ray) -> f32 {
        let oc: vec3::Vec3 = r.origin() - center;
        let a: f32 = vec3::dot(r.direction(), r.direction());
        let b: f32 = 2.0 * vec3::dot(oc, r.direction());
        let c: f32 = vec3::dot(oc, oc) - radius*radius;
        let discriminant: f32 = b*b - 4.0*a*c;
        if discriminant < 0.0 {
            return -1.0;
        }
        else {
            return (-b - discriminant.sqrt()) / (2.0*a);
        }
    }
    
    pub fn color(r1: ray::Ray) -> vec3::Vec3{
        let t = hit_sphere(vec3::Vec3{e: [0.0 ,0.0, -1.0]}, 0.5, r1);
        if t > 0.0 {
            let n = vec3::Vec3::unit_vector(r1.point_at_parameter(t) - vec3::Vec3{e: [0.0, 0.0, -1.0]});
            return 0.5*vec3::Vec3{e: [n.x() + 1.0, n.y() + 1.0, n.z() + 1.0]};
        }
        let unit_direction = vec3::Vec3::unit_vector(r1.direction());
        let t: f32 = (unit_direction.y() + 1.0)*0.5;
        return vec3::Vec3{e: [1.0, 1.0, 1.0]}*(1.0-t) + vec3::Vec3{e: [0.5, 0.7, 1.0]}*t;
    }
    
    let nx: i32 = 200;
    let ny: i32 = 100;
    
    println!("P3\n{} {}\n255", nx, ny);
    
    let lower_left_corner = vec3::Vec3 {e: [-2.0, -1.0, -1.0]};
    let horizontal = vec3::Vec3 {e: [4.0, 0.0, 0.0]};
    let vertical = vec3::Vec3 {e: [0.0, 2.0, 0.0]};
    let origin = vec3::Vec3 {e: [0.0, 0.0, 0.0]};
    
    let mut j: i32 = ny - 1;
    while j > 0 {
        let mut i: i32 = 0;
        while i < nx {
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;
            let r = ray::Ray{a: origin, b: lower_left_corner + u*horizontal + v*vertical};
            let col = color(r);
            let ir: u32 = (255.99*col[0]) as u32;
            let ig: u32 = (255.99*col[1]) as u32;
            let ib: u32 = (255.99*col[2]) as u32;
            println!("{} {} {}", ir, ig, ib);
            // std::cout << ir << " " << ig << " " << ib << "\n";
            i += 1;
        }
        j -= 1;
    }
}

pub fn blue_shader_with_2_sphere_shading(){
    pub fn color<H: Hittable>(r: &Ray, hitable: &H) -> Color3 {
        if let Some(hit_record) = hitable.hit(r, 0.0, std::f32::MAX) {
            &Color3 {
                r: hit_record.normal.x() + 1.0,
                g: hit_record.normal.y() + 1.0,
                b: hit_record.normal.z() + 1.0,
            } * 0.5
        } else {
            let unit_direction : Vec3 = r.direction().unit_vector();
            let t              : f32  = 0.5 * (unit_direction.y() + 1.0);
            &(&Color3 { r: 1.0, g: 1.0, b: 1.0 } * (1.0 - t)) + &(&Color3 {r: 0.5, g: 0.7, b: 1.0} * t)
        }
    }
    
    let mut writer = io::BufWriter::new(io::stdout());

    let nx: i32 = 200;
    let ny: i32 = 100;
    writer.write_all(format!("P3\n{} {}\n255\n", nx, ny).as_bytes()).unwrap();

    let lower_left_corner : Vec3 = Vec3{e: [-2.0, -1.0, -1.0]};
    let horizontal        : Vec3 = Vec3{e: [4.0, 0.0, 0.0]};
    let vertical          : Vec3 = Vec3{e: [0.0, 2.0, 0.0]};
    let origin            : Vec3 = Vec3{e: [0.0, 0.0, 0.0]};
    let hitable           = HittableList { hitables: vec![
        Sphere {center: Vec3{e: [0.0, 0.0, -1.0]}, radius: 0.5},
        Sphere {center: Vec3{e: [0.0, -100.5, -1.0]}, radius: 100.0},
    ]};
    let mut j = ny - 1;
    while j >= 0 {
        for i in 0..nx {
            let u: f32 = i as f32 / nx as f32;
            let v: f32 = j as f32 / ny as f32;
            let r: Ray = Ray{
                a: origin,
                b: (lower_left_corner + (horizontal * u)) + (vertical * v)
            };

            let col: Color3 = color(&r, &hitable);
            writer.write_all(format!("{} {} {}\n", col.ir(), col.ig(), col.ib()).as_bytes()).unwrap();
        }
        j -= 1;
    }
}

pub fn blue_shader_with_2_sphere_shading_with_anti_aliasing(){
    pub fn color<H: Hittable>(r: &Ray, hitable: &H) -> Color3 {
        if let Some(hit_record) = hitable.hit(r, 0.0, std::f32::MAX) {
            &Color3 {
                r: hit_record.normal.x() + 1.0,
                g: hit_record.normal.y() + 1.0,
                b: hit_record.normal.z() + 1.0,
            } * 0.5
        } else {
            let unit_direction : Vec3 = r.direction().unit_vector();
            let t              : f32  = 0.5 * (unit_direction.y() + 1.0);
            &(&Color3 { r: 1.0, g: 1.0, b: 1.0 } * (1.0 - t)) + &(&Color3 {r: 0.5, g: 0.7, b: 1.0} * t)
        }
    }
    
    let mut writer = io::BufWriter::new(io::stdout());

    let nx: i32 = 200;
    let ny: i32 = 100;
    let ns: i32 = 100;
    writer.write_all(format!("P3\n{} {}\n255\n", nx, ny).as_bytes()).unwrap();
    
    let mut random: utils::Random = Default::default();

    let hitable = HittableList { hitables: vec![
        Sphere {center: Vec3{e: [0.0, 0.0, -1.0]}, radius: 0.5},
        Sphere {center: Vec3{e: [0.0, -100.5, -1.0]}, radius: 100.0},
    ]};
    let cam: Camera = Default::default();
    let mut j = ny - 1;
    while j >= 0 {
        for i in 0..nx {
            let mut col = Color3 { r: 0.0, g: 0.0, b: 0.0 };
            for s in 0..ns {
                let u: f32 = (i as f32 + random.f32()) / nx as f32;
                let v: f32 = (j as f32 + random.f32()) / ny as f32;
                let r: Ray = cam.get_ray(u, v);
                col += color(&r, &hitable);
            }
            col /= ns;
            
            writer.write_all(format!("{} {} {}\n", col.ir(), col.ig(), col.ib()).as_bytes()).unwrap();
        }
        j -= 1;
    }
}

pub fn diffused_shader_with_2_spheres(){
    let mut random: utils::Random = Default::default();
    pub fn random_in_unit_sphere(random: &mut utils::Random) -> Vec3 {
        let mut p: Vec3 = Default::default();
        while p.squared_length() >= 1.0 {
            p = 2.0*Vec3{e: [random.f32(), random.f32(), random.f32()]} - Vec3{ e: [1.0, 1.0, 1.0] };
        }
        p
    }
    pub fn color<H: Hittable>(r: &Ray, hitable: &H, random: &mut utils::Random) -> Color3 {
        if let Some(hit_record) = hitable.hit(r, 0.001, std::f32::MAX) {
            let _target = hit_record.p + hit_record.normal + random_in_unit_sphere(random);
            return color(&Ray{a: hit_record.p, b: (_target - hit_record.p)}, hitable, random) * 0.5;
        } else {
            let unit_direction : Vec3 = r.direction().unit_vector();
            let t              : f32  = 0.5 * (unit_direction.y() + 1.0);
            &(&Color3 { r: 1.0, g: 1.0, b: 1.0 } * (1.0 - t)) + &(&Color3 {r: 0.5, g: 0.7, b: 1.0} * t)
        }
    }
    
    let mut writer = io::BufWriter::new(io::stdout());

    let nx: i32 = 200;
    let ny: i32 = 100;
    let ns: i32 = 100;
    writer.write_all(format!("P3\n{} {}\n255\n", nx, ny).as_bytes()).unwrap();
    
    let mut random: utils::Random = Default::default();

    let hitable = HittableList { hitables: vec![
        Sphere {center: Vec3{e: [0.0, 0.0, -1.0]}, radius: 0.5},
        Sphere {center: Vec3{e: [0.0, -100.5, -1.0]}, radius: 100.0},
    ]};
    let cam: Camera = Default::default();
    let mut j = ny - 1;
    while j >= 0 {
        for i in 0..nx {
            let mut col = Color3 { r: 0.0, g: 0.0, b: 0.0 };
            for s in 0..ns {
                let u: f32 = (i as f32 + random.f32()) / nx as f32;
                let v: f32 = (j as f32 + random.f32()) / ny as f32;
                let r: Ray = cam.get_ray(u, v);
                col += color(&r, &hitable, &mut random);
            }
            col /= ns;
            
            writer.write_all(format!("{} {} {}\n", col.g2_ir(), col.g2_ig(), col.g2_ib()).as_bytes()).unwrap();
        }
        j -= 1;
    }
}