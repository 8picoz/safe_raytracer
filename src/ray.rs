use crate::vec3::*;

const TMIN: f32 = 1e-3f;
const TMAX: f32 = 10000;

pub struct Ray {
    origin: Vec3f,
    direction: Vec3f,
}

impl Ray {
    fn new(origin: Vec3f, direction: Vec3f) -> Self {
        Ray { origin, direction }
    }

    fn point_on_ray(distance: f32) -> Vec3f {
        return origin + direction * distance;
    }
}