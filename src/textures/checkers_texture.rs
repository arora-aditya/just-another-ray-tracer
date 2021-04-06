use std::rc::Rc;
use crate::textures::texture::Texture;
use crate::textures::solid_color::SolidColor;
use crate::tracer::color::Color3;
use crate::tracer::vec3::Vec3;

pub struct CheckersTexture {
    pub odd: Rc<dyn Texture>,
    pub even: Rc<dyn Texture>,
}

impl CheckersTexture {
    pub fn new(odd: Color3, even: Color3) -> Self {
        CheckersTexture {
            odd: Rc::new(SolidColor::new(odd)),
            even: Rc::new(SolidColor::new(even)),
        }
    }
}

impl Texture for CheckersTexture {
    fn value(self: &Self, u: f32, v: f32, p: &Vec3) -> Color3 {
        let sines = f64::sin((10.0 * p.x()).into()) * f64::sin((10.0 * p.y()).into()) * f64::sin((10.0 * p.z()).into());
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}