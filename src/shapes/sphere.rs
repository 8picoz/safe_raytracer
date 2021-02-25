use crate::material::Material;
use crate::vec3::*;

#[derive(Debug, Clone)]
pub struct Sphere {
    pub center_position: Vec3f,
    pub radius: f32,
    pub material: Material,
    pub kd: Vec3f,
}

impl Sphere {
    pub fn new(center_position: Vec3f, radius: f32, material: Material, kd: Vec3f) -> Self {
        Sphere {
            center_position,
            radius,
            material,
            kd,
        }
    }
}
