use crate::tracer::ray as ray;
use crate::tracer::vec3 as vec3;

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