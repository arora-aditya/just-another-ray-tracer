use std::path::Path;
use image::{DynamicImage, GenericImageView};
use crate::textures::texture::Texture;
use crate::textures::perlin::Perlin;
use crate::tracer::color::Color3;
use crate::tracer::vec3::Vec3;

pub struct ImageTexture {
    img: DynamicImage
}

impl ImageTexture {
    pub fn new(filename: &Path) -> Self {
        ImageTexture {
            img: image::io::Reader::open(filename).unwrap().decode().unwrap()
        }
    }
}

pub fn clamp<T: PartialOrd>(x: T, min: T, max: T) -> T {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

impl Texture for ImageTexture {
    fn value(self: &Self, u: f32, v: f32, _p: &Vec3) -> Color3 {
        let u = clamp(u, 0.0, 1.0);
        let width = self.img.width();
        let i = clamp((u * width as f32) as u32, 0, width - 1);

        let v = 1.0 - clamp(v, 0.0, 1.0);
        let height = self.img.height();
        let j = clamp((v * height as f32) as u32, 0, height - 1);

        let color_scale = 1.0 / 255.0;
        let pixel = self.img.get_pixel(i, j);

        Color3::new(color_scale * pixel[0] as f32, color_scale * pixel[1] as f32, color_scale * pixel[2] as f32)
    }
}