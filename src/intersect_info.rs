use crate::vec3::*;

#[derive(Debug)]
pub struct IntersectInfo {
    pub distance: f32,
    pub point: Vec3f,
    pub normal: Vec3f,
}

impl IntersectInfo {
    pub fn new(distance: f32, point: Vec3f, normal: Vec3f) -> Self {
        IntersectInfo {
            distance,
            point,
            normal,
        }
    }
}
