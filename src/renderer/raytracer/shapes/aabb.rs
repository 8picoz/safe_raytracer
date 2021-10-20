use core::f32;
use std::usize;

use crate::renderer::raytracer::ray::Ray;
use crate::vec3::Vec3;
use crate::vec3::axis::*;
use crate::vec3::Vec3f;

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug)]
pub struct AABB<T: Copy + Send + Sync> {
    min: Vec3<T>,
    max: Vec3<T>,
}

impl AABB<f32> {
    pub fn new(min: Vec3<f32>, max: Vec3<f32>) -> Self {
        Self { min, max }
    }

    pub fn new_max_bound() -> Self {

        let min = Vec3::from(f32::MIN);
        let max = Vec3::from(f32::MAX);

        Self { min, max }
    }
    
    pub fn new_zero_bound() -> Self {
        let zero = Vec3::from(0.0);

        Self { min: zero, max: zero }
    }

    pub fn merge_aabb(&self, other: Self) -> Self {
        let min = self.min.cmp_each_min_value(other.min);
        let max = self.max.cmp_each_max_value(other.max);

        AABB::new(min, max)
    }

    pub fn merge_point(&self, other: Vec3<f32>) -> Self {
        let min = self.min.cmp_each_min_value(other);
        let max = self.max.cmp_each_max_value(other);

        AABB::new(min, max)
    }

    pub fn center(&self) -> Vec3<f32> {
        (self.min + self.max) * 0.5
    }

    pub fn longest_axis(&self) -> Axis {
        let length = self.max - self.min;

        if length.x >= length.y && length.x >= length.z {
            return Axis::X;
        } else if length.y >= length.x && length.y >= length.z {
            return Axis::Y;
        }

        Axis::Z
    }

    pub fn intersect(&self, ray: &Ray, dir_inv: Vec3f, dir_inv_sign: Vec3<usize>) -> bool {

        let mut t_min = (self.get_bounds_idx(dir_inv_sign.x).x - ray.origin.x) * dir_inv.x;
        let mut t_max = (self.get_bounds_idx(1 - dir_inv_sign.x).x - ray.origin.x) * dir_inv.x;
        let t_y_min = (self.get_bounds_idx(dir_inv_sign.y).y - ray.origin.y) * dir_inv.y;
        let t_y_max = (self.get_bounds_idx(1 - dir_inv_sign.y).y - ray.origin.y) * dir_inv.y;
        if t_min > t_y_max || t_y_min > t_max {
            return false;
        }
        if t_y_min > t_min {
            t_min = t_y_min;
        }
        if t_y_max < t_max {
            t_max = t_y_max;
        }
        
        let t_z_min = (self.get_bounds_idx(dir_inv_sign.z).z - ray.origin.z) * dir_inv.z;
        let t_z_max = (self.get_bounds_idx(1 - dir_inv_sign.z).z - ray.origin.z) * dir_inv.z;
        if t_min > t_z_max || t_z_min > t_max {
            return false;
        }
        if t_z_min > t_min {
            t_min = t_z_min;
        }
        if t_z_max < t_max {
            t_max = t_z_max;
        }
        
        t_min < ray.t_max && t_max > ray.t_min
    }

    fn get_bounds_idx(&self, value: usize) -> Vec3f {
        match value {
            0 => self.min,
            1 => self.max,
            //デフォルトがあることがおかしい
            _ => panic!("Out-of-range access"),
        }
    }
}