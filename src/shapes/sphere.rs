use crate::vec3::Vec3f;

use super::bsdf::BSDF;

#[derive(Debug)]
pub struct Sphere {
    pub center_position: Vec3f,
    pub radius: f32,
    pub bsdf: BSDF,
}

impl Sphere {
    pub fn new(center_position: Vec3f, radius: f32, bsdf: BSDF) -> Self {
        Sphere {
            center_position,
            radius,
            bsdf,
        }
    }
}
