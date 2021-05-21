use crate::intersect_info::IntersectInfo;
use crate::ray::Ray;
use crate::shapes::obj::Obj;
use crate::shapes::sphere::Sphere;
use crate::shapes::triangle::Triangle;
use crate::shapes::Shapes;
use crate::vec3::Vec3f;

#[allow(clippy::upper_case_acronyms)]
pub struct BVH {
    //Shapesの実態の所有権はScene(BVH)が持つべき
    //IntersectInfoなどのライフタイムはScene(BVH)が消えたらそもそも存在できない
    shapes: Vec<Shapes>,
    pub directional_light: Vec3f,
}

impl BVH {
    pub fn new(directional_light: Vec3f) -> Self {
        BVH {
            shapes: Vec::new(),
            directional_light,
        }
    }

    //TODO: addをgenericに
    pub fn add_sphere(&mut self, sphere: Sphere) {
        self.shapes.push(Shapes::Sphere(sphere));
    }

    pub fn add_triangle(&mut self, triangle: Triangle) {
        self.shapes.push(Shapes::Triangle(triangle))
    }

    pub fn add_obj(&mut self, obj: Obj) {
        for shape in obj.triangles {
            self.shapes.push(shape);
        }
    }

    pub fn collision_detect(&self, ray: &Ray) -> Option<IntersectInfo> {
        let mut infos = Vec::new();

        for shape in self.shapes.iter() {
            if let Some(info) = shape.collision_detect(ray) {
                infos.push(info);
            }
        }

        if infos.is_empty() {
            return None;
        }

        Some(
            infos
                .into_iter()
                .min_by(|a, b| {
                    a.distance
                        .partial_cmp(&b.distance)
                        .expect("failed to compare")
                })
                .expect("failed to pick max value"),
        )
    }
}
