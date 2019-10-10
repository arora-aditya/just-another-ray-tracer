// mod ray;
mod tracer;

fn ppm_test(){
    let mut nx: u32 = 200;
    let mut ny: u32 = 100;
    
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

fn vec_test(){
    let mut v1 = tracer::vec3::Vec3 { e: [0.0,1.0,2.0] };
    println!("v1: {}", v1);
    let v2 = tracer::vec3::Vec3 { e: [10.0,11.0,12.0] };
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
    println!("v1 x v2 {}", tracer::vec3::cross(v1, v2));
    v1.make_unit_vector();
    println!("unit v1 {}", v1);
}

fn main() {
    // ppm_test();
    vec_test();
}
