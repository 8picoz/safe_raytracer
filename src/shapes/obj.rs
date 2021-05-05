use tobj::Model;

use crate::material::Material;
use crate::vec3::Vec3f;

#[derive(Debug)]
pub struct Obj {
    pub center_position: Vec3f,
    pub material: Material,
    pub kd: Vec3f,
    models: Vec<Model>,
}

impl Obj {
    pub fn new(file_path: &str, center_position: Vec3f, material: Material, kd: Vec3f) -> Self {
        let (models, _) =
            tobj::load_obj(file_path, false).unwrap_or_else(|_| panic!("Failed to load file"));

        //AABB計算

        Obj {
            center_position,
            material,
            kd,
            models,
        }
    }
}
