use crate::textures::texture::Texture;
use crate::tracer::color::Color3;
use crate::tracer::vec3::Vec3;

pub struct SolidColor {
    pub color: Color3,
}

impl SolidColor {
    pub fn new(color_value: Color3) -> Self {
        SolidColor {
            color: color_value,
        }
    }
}

impl Texture for SolidColor {
    fn value(&self, u: f32, v: f32, p: &Vec3) -> Color3 {
        self.color.clone()
    }
}