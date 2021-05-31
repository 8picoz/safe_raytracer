use crate::vec3::Vec3;
use crate::vec3::Vec3f;

use super::aabb::AABB;
use super::bsdf::BSDF;

#[derive(Debug, Clone)]
pub struct Triangle {
    pub v0: Vec3f,
    pub v1: Vec3f,
    pub v2: Vec3f,
    pub bsdf: BSDF,
    center_position: Option<Vec3f>,
}

impl Triangle {
    pub fn new(v0: Vec3f, v1: Vec3f, v2: Vec3f, bsdf: BSDF) -> Self {
        Triangle {
            v0,
            v1,
            v2,
            bsdf,
            center_position: None,
        }
    }

    pub fn get_center_position(&mut self) -> Vec3f {
        match self.center_position {
            Some(position) => position,
            None => {
                //とりあえず重心
                Vec3f::new(
                    (self.v0.x + self.v1.x + self.v2.x) / 3.,
                    (self.v0.y + self.v1.y + self.v2.y) / 3.,
                    (self.v0.z + self.v1.z + self.v2.z) / 3.,
                )
            }
        }
    }

    pub fn calc_aabb(&self) -> AABB<f32> {
        let vertices = [self.v0, self.v1, self.v2];
        let (x_max, x_min) = vertices.iter().map(|i| i.x).fold((f32::MIN, f32::MAX), |(m_max, m_min), v| (v.max(m_max), v.min(m_min)));
        let (y_max, y_min) = vertices.iter().map(|i| i.y).fold((f32::MIN, f32::MAX), |(m_max, m_min), v| (v.max(m_max), v.min(m_min)));
        let (z_max, z_min) = vertices.iter().map(|i| i.z).fold((f32::MIN, f32::MAX), |(m_max, m_min), v| (v.max(m_max), v.min(m_min)));

        let max = Vec3::new(x_max, y_max, z_max);
        let min = Vec3::new(x_min, y_min, z_min);

        AABB::new(min, max)
    }
}
