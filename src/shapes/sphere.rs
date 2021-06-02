use crate::vec3::Vec3f;

use super::aabb::AABB;
use super::bsdf::BSDF;

#[derive(Debug, Clone)]
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

    pub fn calc_aabb(&self) -> AABB<f32> {
        //vec3のイテレーターにto_vec3を作りたい
        let max = Vec3f::new(self.center_position.x + self.radius, self.center_position.y + self.radius, self.center_position.z + self.radius);
        let min = Vec3f::new(self.center_position.x - self.radius, self.center_position.y - self.radius, self.center_position.z - self.radius);

        AABB::new(min, max)
    }
}
