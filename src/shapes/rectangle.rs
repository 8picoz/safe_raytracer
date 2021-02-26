use crate::material::*;
use crate::ray::TMIN;
use crate::vec3::*;

#[derive(Debug)]
pub struct Rectangle {
    pub lu: Vec3f,
    pub ld: Vec3f,
    pub rd: Vec3f,
    pub ru: Vec3f,
    pub material: Material,
    pub kd: Vec3f,
    x_max: f32,
    y_max: f32,
    z_max: f32,
    x_min: f32,
    y_min: f32,
    z_min: f32,
}

impl Rectangle {
    pub fn new(lu: Vec3f, ld: Vec3f, rd: Vec3f, ru: Vec3f, material: Material, kd: Vec3f) -> Self {
        let vertices = vec![lu, ld, rd, ru];

        let x_max = vertices
            .iter()
            .map(|item| item.x)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap()
            + TMIN;
        let y_max = vertices
            .iter()
            .map(|item| item.y)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap()
            + TMIN;
        let z_max = vertices
            .iter()
            .map(|item| item.z)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap()
            + TMIN;

        let x_min = vertices
            .iter()
            .map(|item| item.x)
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap()
            - TMIN;
        let y_min = vertices
            .iter()
            .map(|item| item.y)
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap()
            - TMIN;
        let z_min = vertices
            .iter()
            .map(|item| item.z)
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap()
            - TMIN;

        Rectangle {
            lu,
            ld,
            rd,
            ru,
            material,
            kd,
            x_max,
            y_max,
            z_max,
            x_min,
            y_min,
            z_min,
        }
    }

    pub fn get_center_position(&self) -> Vec3f {
        (self.lu + self.rd) / 2.0
    }

    pub fn point_is_inside(&self, point: Vec3f) -> bool {
        (self.x_min <= point.x && point.x <= self.x_max)
            && (self.y_min <= point.y && point.y <= self.y_max)
            && (self.z_min <= point.z && point.z <= self.z_max)
    }
}
