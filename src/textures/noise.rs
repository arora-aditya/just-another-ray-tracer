use crate::textures::texture::Texture;
use crate::textures::perlin::Perlin;
use crate::tracer::color::Color3;
use crate::tracer::vec3::Vec3;

pub struct NoiseTexture {
    noise: Perlin,
    scale: f32,
}

impl NoiseTexture {
    pub fn new(scale: f32) -> Self {
        NoiseTexture {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(self: &Self, _u: f32, _v: f32, p: &Vec3) -> Color3 {
        Color3::new(1.0, 1.0, 1.0) * 0.5 * (1.0 + f32::sin(self.scale * p.z() + 10.0 * self.noise.turb(p, Option::None)))
    }
}
