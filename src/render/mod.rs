use std::io;
use std::io::Write;

use crate::utils;

use crate::tracer::ray::{self, Ray};
use crate::tracer::vec3::{self, Vec3};
use crate::tracer::color::Color3;
use crate::objects::hittable_list::HittableList;
use crate::objects::hittable;
use crate::material;
use crate::material::metal::MetalMaterial;
use crate::material::lambert::LambertMaterial;
use crate::material::dielectric::DielectricMaterial;
use crate::objects::sphere::Sphere;
use crate::objects::hittable::Hittable;
use crate::camera::camera::{self, Camera};

pub fn change_camera_dielectric_with_2_spheres(){
    pub fn random_in_unit_sphere(random: &mut utils::Random) -> Vec3 {
        let mut p: Vec3 = Default::default();
        while p.squared_length() >= 1.0 {
            p = 2.0*Vec3{e: [random.f32(), random.f32(), random.f32()]} - Vec3{ e: [1.0, 1.0, 1.0] };
        }
        p
    }
    pub fn color<H: Hittable>(r: &Ray, hitable: &H, random: &mut utils::Random, depth: i32) -> Color3 {
        if let Some(hit_record) = hitable.hit(r, 0.001, std::f32::MAX) {
            if depth < 50 {
                if let Some(scatter_record) = hit_record.material.scatter(random, *r, hit_record) {
                    let col = color(&scatter_record.scattered, hitable, random, depth+1);
                    let attenuation = scatter_record.attenuation;
                    Color3 {
                        r: col.r * attenuation.r,
                        g: col.g * attenuation.g,
                        b: col.b * attenuation.b
                    }
                } else {
                    Color3 {r: 0.0, g: 0.0, b: 0.0}
                }
            } else {
                Color3 {r: 0.0, g: 0.0, b: 0.0}
            }
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
    let r: f32 = (std::f32::consts::PI/4.0).cos();
    let hitable = HittableList { hitables: vec![
        Sphere {
            center: Vec3{e: [0.0, 0.0, -1.0]}, 
            radius: 0.5,
            material: &LambertMaterial{
                albedo: Color3{r: 0.1, g: 0.2, b: 0.5
            }},
        },
        Sphere {
            center: Vec3{e: [0.0, -100.5, -1.0]}, 
            radius: 100.0,
            material: &LambertMaterial{
                albedo: Color3{r: 0.8, g: 0.8, b: 0.0
            }},
        },
        Sphere {
            center: Vec3{e: [1.0, 0.0, -1.0]}, 
            radius: -0.45,
            material: &DielectricMaterial{
                ref_idx: 1.5,
            },
        },
        Sphere {
            center: Vec3{e: [1.0, 0.0, -1.0]}, 
            radius: 0.5,
            material: &DielectricMaterial{
                ref_idx: 1.5,
            },
        },
        Sphere {
            center: Vec3{e: [-1.0, 0.0, -1.0]}, 
            radius: 0.5,
            material: &MetalMaterial{
                albedo: Color3{r: 0.8, g: 0.6, b: 0.8},
                f: 0.0,
            },
        },
    ]};
    
    let lookfrom: Vec3 = vec3::new(3,3,2);
    let lookat: Vec3 = vec3::new(0,0,-1);
    let dist_to_focus:f32 = (lookfrom-lookat).length();
    let aperture: f32 = 2.0;
    let cam: Camera = Camera::new(
            lookfrom, 
            lookat, 
            vec3::new(0,1,0), 
            20.0, 
            nx as f32/ny as f32, 
            aperture, 
            dist_to_focus);
    let mut j = ny - 1;
    while j >= 0 {
        for i in 0..nx {
            let mut col = Color3 { r: 0.0, g: 0.0, b: 0.0 };
            for s in 0..ns {
                let u: f32 = (i as f32 + random.f32()) / nx as f32;
                let v: f32 = (j as f32 + random.f32()) / ny as f32;
                let r: Ray = cam.get_ray(u, v, &mut random);
                col += color(&r, &hitable, &mut random, 0);
            }
            col /= ns;
            
            writer.write_all(format!("{} {} {}\n", col.g2_ir(), col.g2_ig(), col.g2_ib()).as_bytes()).unwrap();
        }
        j -= 1;
    }
}