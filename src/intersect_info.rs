use crate::sphere::Sphere;
use crate::vec3::*;

#[derive(Debug)]
pub struct IntersectInfo<'a> {
    pub distance: f32,
    pub point: Vec3f,
    pub normal: Vec3f,
    pub target_sphere: &'a Sphere,
}

impl<'a> IntersectInfo<'a> {
    pub fn new(distance: f32, point: Vec3f, normal: Vec3f, target_sphere: &'a Sphere) -> Self {
        IntersectInfo {
            distance,
            point,
            normal,
            target_sphere,
        }
    }
}
