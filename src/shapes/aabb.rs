use core::f32;

use crate::vec3::Vec3;

#[allow(clippy::upper_case_acronyms)]
pub struct AABB<T: Copy + Send + Sync> {
    max: Vec3<T>,
    min: Vec3<T>,
}

impl AABB<f32> {
    pub fn new(max: Vec3<f32>, min: Vec3<f32>) -> Self {
        Self { max, min }
    }

    pub fn new_max_bound() -> Self {

        let max = Vec3::from(f32::MAX);
        let min = Vec3::from(f32::MIN);

        Self { max, min }
    }
    
    pub fn new_min_bound() -> Self {
        let zero = Vec3::from(0.0);

        Self { max: zero, min: zero }
    }

    pub fn merge_aabb(&self, other: Self) -> Self {
        let max = self.max.cmp_each_max_value(other.max);
        let min = self.min.cmp_each_min_value(other.min);

        AABB::new(max, min)
    }

    pub fn merge_point(&self, other: Vec3<f32>) -> Self {
        let max = self.max.cmp_each_max_value(other);
        let min = self.min.cmp_each_min_value(other);

        AABB::new(max, min)
    }

    pub fn center(&self) -> Vec3<f32> {
        (self.max * self.min) * 0.5
    }
}