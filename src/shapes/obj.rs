use core::panic;
use std::usize;

use crate::shapes::triangle::Triangle;
use crate::vec3::Vec3;
use crate::vec3::Vec3f;

use super::Shapes;
use super::bsdf::BSDF;

#[derive(Debug)]
pub struct Obj {
    pub center_position: Vec3f,
    pub triangles: Vec<Shapes>,
}

const FAILED_TO_GET_FACE_INDICES: fn() -> &'static &'static u32 =
    || panic!("Failed to get face_indices");
const FAILED_TO_GET_MESH_POSTION: fn() -> &'static f32 = || panic!("Failed to get mesh position");

impl Obj {
    pub fn new(file_path: &str, center_position: Vec3f, bsdf: BSDF) -> Self {
        let (models, _) =
            tobj::load_obj(file_path, false).unwrap_or_else(|_| panic!("Failed to load file"));

        //AABB計算

        let mut triangles: Vec<Shapes> = vec![];
        for (_, m) in models.iter().enumerate() {
            let mesh = &m.mesh;

            let mut next_face = 0;
            for f in 0..mesh.num_face_indices.len() {
                let end = next_face + mesh.num_face_indices[f] as usize;
                let face_indices: Vec<_> = mesh.indices[next_face..end].iter().collect();

                let v0_index = 3 * **face_indices
                    .get(0)
                    .unwrap_or_else(FAILED_TO_GET_FACE_INDICES)
                    as usize;
                let v0 = Vec3::new(
                    *mesh
                        .positions
                        .get(v0_index)
                        .unwrap_or_else(FAILED_TO_GET_MESH_POSTION),
                    *mesh
                        .positions
                        .get(v0_index + 1)
                        .unwrap_or_else(FAILED_TO_GET_MESH_POSTION),
                    *mesh
                        .positions
                        .get(v0_index + 2)
                        .unwrap_or_else(FAILED_TO_GET_MESH_POSTION),
                );

                let v1_index = 3 * **face_indices
                    .get(1)
                    .unwrap_or_else(FAILED_TO_GET_FACE_INDICES)
                    as usize;
                let v1 = Vec3::new(
                    *mesh
                        .positions
                        .get(v1_index)
                        .unwrap_or_else(FAILED_TO_GET_MESH_POSTION),
                    *mesh
                        .positions
                        .get(v1_index + 1)
                        .unwrap_or_else(FAILED_TO_GET_MESH_POSTION),
                    *mesh
                        .positions
                        .get(v1_index + 2)
                        .unwrap_or_else(FAILED_TO_GET_MESH_POSTION),
                );

                let v2_index = 3 * **face_indices
                    .get(2)
                    .unwrap_or_else(FAILED_TO_GET_FACE_INDICES)
                    as usize;
                let v2 = Vec3::new(
                    *mesh
                        .positions
                        .get(v2_index)
                        .unwrap_or_else(FAILED_TO_GET_MESH_POSTION),
                    *mesh
                        .positions
                        .get(v2_index + 1)
                        .unwrap_or_else(FAILED_TO_GET_MESH_POSTION),
                    *mesh
                        .positions
                        .get(v2_index + 2)
                        .unwrap_or_else(FAILED_TO_GET_MESH_POSTION),
                );

                //NOTE: ここのbsdfをRcでどうにかする
                let triangle = Shapes::Triangle(Triangle::new(
                    v0 + center_position,
                    v1 + center_position,
                    v2 + center_position,
                    bsdf.clone(),
                ));

                triangles.push(triangle);
                next_face = end;
            }
        }

        Obj {
            center_position,
            triangles,
        }
    }

    pub fn get_bsdf(&self) -> &BSDF {
        self.triangles.get(0).unwrap().get_bsdf()
    }
}
