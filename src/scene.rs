use crate::intersect_info::*;
use crate::ray::*;
use crate::shapes::Shape;
use crate::vec3::Vec3f;

pub struct Scene<T: Shape> {
    shapes: Vec<T>,
    pub directional_light: Vec3f,
}

impl<T: Shape> Scene<T> {
    pub fn new_without_spheres(directional_light: Vec3f) -> Self {
        Scene {
            shapes: Vec::new(),
            directional_light,
        }
    }

    pub fn new_with_spheres(shapes: Vec<T>, directional_light: Vec3f) -> Self {
        Scene {
            shapes,
            directional_light,
        }
    }

    pub fn add_shape(&mut self, shape: T) {
        self.shapes.push(shape);
    }

    pub fn collision_detect(&self, ray: &Ray) -> Option<IntersectInfo<T>> {
        let mut infos = Vec::new();

        for shape in self.shapes.iter() {
            if let Some(info) = shape.collision_detect(ray) {
                let info = IntersectInfo::new(
                    info.distance,
                    info.point,
                    (info.point - shape.get_center_potision()).normalized(),
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
