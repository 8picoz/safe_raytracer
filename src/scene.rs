use crate::intersect_info::*;
use crate::ray::*;
use crate::sphere::*;
use crate::vec3::Vec3f;

pub struct Scene {
    spheres: Vec<Sphere>,
    pub directional_light: Vec3f,
}

impl Scene {
    pub fn new_without_spheres(directional_light: Vec3f) -> Self {
        Scene {
            spheres: Vec::new(),
            directional_light,
        }
    }

    pub fn new_with_spheres(spheres: Vec<Sphere>, directional_light: Vec3f) -> Self {
        Scene {
            spheres,
            directional_light,
        }
    }

    pub fn add_sphere(&mut self, sphere: Sphere) {
        self.spheres.push(sphere);
    }

    pub fn collision_detect(&self, ray: &Ray) -> Option<IntersectInfo> {
        let mut infos = Vec::new();

        for sphere in self.spheres.iter() {
            if let Some(info) = sphere.collision_detect(ray) {
                let info = IntersectInfo::new(
                    info.distance,
                    info.point,
                    (info.point - sphere.point).normalized(),
                    sphere,
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
