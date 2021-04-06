use crate::tracer::vec3::Vec3;
use crate::tracer::color::Color3;
pub trait Texture {
    fn value(&self, u: f32, v: f32, p: &Vec3) -> Color3;
}