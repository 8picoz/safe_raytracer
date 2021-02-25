use crate::intersect_info::*;
use crate::ray::*;
use crate::shapes::rectangle::Rectangle;
use crate::shapes::sphere::Sphere;
use crate::shapes::Shapes;
use crate::vec3::Vec3f;

pub struct Scene {
    //Shapesの実態の所有権はSceneが持つべき
    //IntersectInfoなどのライフタイムはSceneが消えたらそもそも存在できない
    shapes: Vec<Shapes>,
    pub directional_light: Vec3f,
}

impl Scene {
    pub fn new(directional_light: Vec3f) -> Self {
        Scene {
            shapes: Vec::new(),
            directional_light,
        }
    }

    pub fn add_sphere(&mut self, sphere: Sphere) {
        self.shapes.push(Shapes::Sphere(sphere));
    }

    pub fn add_rectangle(&mut self, rect: Rectangle) {
        self.shapes.push(Shapes::Rectangle(rect));
    }

    pub fn collision_detect(&self, ray: &Ray) -> Option<IntersectInfo> {
        let mut infos = Vec::new();

        for shape in self.shapes.iter() {
            if let Some(info) = shape.collision_detect(ray) {
                let info = IntersectInfo::new(
                    info.distance,
                    info.point,
                    (info.point - shape.get_center_position()).normalized(),
                    shape,
                );

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
