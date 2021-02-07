use crate::vec3::*;

const TMIN: f32 = 1e-3;
const TMAX: f32 = 10000.0;

pub struct Ray {
    pub origin: Vec3f,
    pub direction: Vec3f,
}

impl Ray {
    pub fn new(origin: Vec3f, direction: Vec3f) -> Self {
        Ray { origin, direction }
    }

    pub fn point_on_ray(&self, distance: f32) -> Vec3f {
        self.origin + self.direction * distance
    }
}
