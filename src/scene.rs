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
            let c_to_o = ray.origin - sphere.point;
            let b = ray.direction.dot(c_to_o);
            let c = c_to_o.sqr_magnitude() - num::pow(sphere.radius, 2);
            let D = num::pow(b, 2) - c;

            if D < 0.0 {
                continue;
            }

            let mut ans = -b - D.sqrt();
            if ans < TMIN || TMAX < ans {
                ans = -b + D.sqrt();

                if ans < TMIN || TMAX < ans {
                    continue;
                }
            }

            let hit_position = ray.point_on_ray(ans);

            let info = IntersectInfo::new(
                ans,
                hit_position,
                (hit_position - sphere.point).normalized(),
                sphere,
            );

            infos.push(info);
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
