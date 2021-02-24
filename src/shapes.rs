use crate::intersect_info::IntersectInfo;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::*;
pub mod sphere;

pub trait Shape {
    type Output: Shape;
    fn collision_detect(&self, ray: &Ray) -> Option<IntersectInfo<Self::Output>>;

    fn get_center_potision(&self) -> Vec3f;
    fn get_material(&self) -> Material;
    fn get_kd(&self) -> Color;
}