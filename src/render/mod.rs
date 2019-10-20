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
use crate::objects::sphere::{Sphere, MovingSphere, SphereThing};
use crate::objects::hittable::Hittable;
use crate::camera::camera::{self, Camera};

pub fn random_hitable(random: &mut utils::Random) -> HittableList<SphereThing> {
    let n: i32 = 500;
    let mut list: Vec<SphereThing> = std::vec::Vec::new();
    list.push(SphereThing::Fixed(Sphere {
        center: Vec3{e: [0.0, -1000.0, 0.0]}, 
        radius: 1000.0,
        material: Box::new(LambertMaterial{
            albedo: Color3{r: 0.5, g: 0.5, b: 0.5}
        }),
    }));
    let i: i32 = 1;
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f32 = random.f32();
            let center: Vec3 = Vec3{e: [(a as f32)+0.9*random.f32(),0.2,(b as f32)+0.9*random.f32()]};
            if ((center-Vec3{e: [4.0,0.2,0.0]}).length() > 0.9) {
                if choose_mat < 0.8 {  // diffuse
                    list.push(SphereThing::Moving(MovingSphere{
                            center0: center, 
                            center1: center + Vec3{e: [0.0, 0.5*random.f32(), 0.0]}, 
                            radius: 0.2,
                            material: Box::new(LambertMaterial{
                                albedo: Color3{
                                            r: random.f32()*random.f32(), 
                                            g: random.f32()*random.f32(), 
                                            b: random.f32()*random.f32()
                                        }
                            }),
                            time0: 0.0,
                            time1: 1.0,
                        }));
                }
                else if choose_mat < 0.95 { // metal
                    list.push(SphereThing::Fixed(Sphere{
                            center: center, 
                            radius: 0.2,
                            material: Box::new(MetalMaterial{
                                albedo: Color3{
                                            r: 0.5*(1.0 + random.f32()), 
                                            g: 0.5*(1.0 + random.f32()), 
                                            b: 0.5*(1.0 + random.f32()),
                                        },
                                f: 0.5*random.f32(),
                            }),
                    }));
                }
                else {  // glass
                    list.push(SphereThing::Fixed(Sphere{
                            center: center, 
                            radius: 0.2,
                            material: Box::new(DielectricMaterial{
                                ref_idx: 1.5,
                            }),
                    }));
                }
            }
        }
    }

    list.push(SphereThing::Fixed(Sphere{
            center: vec3::new(0, 1, 0), 
            radius: 1.0,
            material: Box::new(DielectricMaterial{
                ref_idx: 1.5,
            }),
    }));
    list.push(SphereThing::Fixed(Sphere{
            center: vec3::new(4, 1, 0), 
            radius: 0.2,
            material: Box::new(MetalMaterial{
                albedo: Color3{
                            r: 0.7, 
                            g: 0.6, 
                            b: 0.5,
                        },
                f: 0.0,
            }),
    }));
    list.push(SphereThing::Fixed(Sphere{
            center: vec3::new(-4, 1, 0), 
            radius: 1.0,
            material: Box::new(LambertMaterial{
                albedo: Color3{
                            r: 0.4, 
                            g: 0.2, 
                            b: 0.1
                        }
            })
    }));

    return HittableList {
        hitables: list
    };
}

pub fn book_cover(){
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

    let nx: i32 = 750;
    let ny: i32 = 375;
    let ns: i32 = 100;
    writer.write_all(format!("P3\n{} {}\n255\n", nx, ny).as_bytes()).unwrap();
    
    let mut random: utils::Random = Default::default();
    let r: f32 = (std::f32::consts::PI/4.0).cos();
    let hitable = random_hitable(&mut random);
    
    let lookfrom: Vec3 = vec3::new(13, 2, 3);
    let lookat: Vec3 = vec3::new(0,0,0);
    let dist_to_focus:f32 = 10.0;
    let aperture: f32 = 0.1;
    let cam: Camera = Camera::new(
            lookfrom, 
            lookat, 
            vec3::new(0,1,0), 
            20.0, 
            nx as f32/ny as f32, 
            aperture, 
            dist_to_focus,
            0.0,
            1.0,
        );
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