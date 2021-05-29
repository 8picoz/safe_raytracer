use crate::vec3::Vec3f;

#[derive(Clone)]
pub struct Ray {
    pub origin: Vec3f,
    pub direction: Vec3f,
    pub t_min: f32,
    pub t_max: f32,
}

impl Ray {
    pub fn new(origin: Vec3f, direction: Vec3f) -> Self {
        Ray { origin, direction, t_min: 1e-3, t_max: f32::MAX }
    }

    pub fn point_on_ray(&self, distance: f32) -> Vec3f {
        self.origin + self.direction * distance
    }
}
