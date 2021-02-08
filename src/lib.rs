pub mod image;
pub mod intersect_info;
pub mod material;
pub mod pinhole_camera;
pub mod ray;
pub mod scene;
pub mod sphere;
pub mod vec3;

#[cfg(test)]
mod tests;

use std::u32;

use material::Material;
use ray::*;
use scene::*;
use vec3::*;

pub const MAX_DEPTH: u32 = 100;

pub fn raytrace(camera_ray: Ray, scene: &Scene, index: u32) -> Color {
    if 100 <= index {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    if let Some(info) = scene.collision_detect(&camera_ray) {
        match info.target_sphere.material {
            Material::Mirror => {
                return raytrace(
                    Ray::new(
                        info.point,
                        refrect(camera_ray.direction * -1.0, info.normal).normalized(),
                    ),
                    scene,
                    index + 1,
                );
            }
            Material::Glass => (),
            _ => (),
        }

        let directional_light_ray = Ray::new(info.point, scene.directional_light);
        if scene.collision_detect(&directional_light_ray).is_none() {
            return info.target_sphere.rgb * scene.directional_light.dot(info.normal).max(0.0);
        }
    }

    Vec3::new(0.0, 0.0, 0.0)
}

fn refrect(vec: Vec3f, normal_of_point: Vec3f) -> Vec3f {
    vec * -1.0 + normal_of_point * vec.dot(normal_of_point) * 2.0
}
