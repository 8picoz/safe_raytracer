use crate::material::Material;
use crate::vec3::Vec3f;

#[derive(Debug)]
pub struct Triangle {
    pub v0: Vec3f,
    pub v1: Vec3f,
    pub v2: Vec3f,
    pub material: Material,
    pub kd: Vec3f,
    center_position: Option<Vec3f>,
}

impl Triangle {
    pub fn new(v0: Vec3f, v1: Vec3f, v2: Vec3f, material: Material, kd: Vec3f) -> Self {
        Triangle {
            v0,
            v1,
            v2,
            material,
            kd,
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
}
