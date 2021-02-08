pub mod image;
pub mod intersect_info;
pub mod pinhole_camera;
pub mod ray;
pub mod scene;
pub mod sphere;
pub mod vec3;

#[cfg(test)]
mod tests;

use std::u32;

use ray::*;
use scene::*;
use vec3::*;

pub const MAX_DEPTH: u32 = 100;

pub fn raytrace(camera_ray: Ray, scene: &Scene, directional_light: Vec3f, index: u32) -> Color {
    if let Some(info) = scene.collision_detect(camera_ray) {
        match info.target_sphere.material {
            _ => (),
        }

        let directional_light_ray = Ray::new(info.point, directional_light);
        if scene.collision_detect(directional_light_ray).is_none() {
            return info.target_sphere.rgb * directional_light.dot(info.normal).max(0.0);
        } else {
            return Vec3::new(0.0, 0.0, 0.0) * info.target_sphere.rgb;
        }
    }

    Vec3::new(0.0, 0.0, 0.0)
}
