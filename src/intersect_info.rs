use crate::shapes::Shape;
use crate::vec3::*;

#[derive(Debug)]
pub struct IntersectInfo<'a, T: Shape> {
    pub distance: f32,
    pub point: Vec3f,
    pub normal: Vec3f,
    pub target: &'a T,
}

impl<'a, T: Shape> IntersectInfo<'a, T> {
    pub fn new(distance: f32, point: Vec3f, normal: Vec3f, target: &'a T) -> Self {
        IntersectInfo {
            distance,
            point,
            normal,
            target,
        }
    }
}
