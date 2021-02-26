use crate::shapes::Shapes;
use crate::vec3::Vec3f;

pub struct IntersectInfo<'a> {
    pub distance: f32,
    pub point: Vec3f,
    pub normal: Vec3f,
    pub target: &'a Shapes,
}

impl<'a> IntersectInfo<'a> {
    pub fn new(distance: f32, point: Vec3f, normal: Vec3f, target: &'a Shapes) -> Self {
        IntersectInfo {
            distance,
            point,
            normal,
            target,
        }
    }
}
