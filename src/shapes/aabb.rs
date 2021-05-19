use core::f32;

use crate::vec3::Vec3;

#[allow(clippy::upper_case_acronyms)]
pub struct AABB {
    min: Vec3<f32>,
    max: Vec3<f32>,
}

impl AABB {
    pub fn new(max: Vec3<f32>, min: Vec3<f32>) -> Self {
        AABB { max, min }
    }

    pub fn new_max_bound() -> Self {

        let max = Vec3::from(f32::MAX);
        let min = Vec3::from(f32::MIN);

        AABB { max, min }
    }
}