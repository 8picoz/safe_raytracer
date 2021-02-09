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

pub struct Raytracer<'a> {
    max_depth: u32,
    scene: &'a Scene,
}

impl<'a> Raytracer<'a> {
    pub fn new(max_depth: u32, scene: &'a Scene) -> Self {
        Raytracer { max_depth, scene }
    }

    pub fn raytrace(&self, camera_ray: Ray, index: u32) -> Color {
        if self.max_depth <= index {
            return Vec3::new(0.0, 0.0, 0.0);
        }

        if let Some(info) = self.scene.collision_detect(&camera_ray) {
            match info.target_sphere.material {
                Material::Mirror => {
                    return self.raytrace(
                        Ray::new(
                            info.point,
                            Raytracer::reflect(camera_ray.direction * -1.0, info.normal)
                                .normalized(),
                        ),
                        index + 1,
                    );
                }
                Material::Glass => {}
                _ => (),
            }

            let directional_light_ray = Ray::new(info.point, self.scene.directional_light);
            if self
                .scene
                .collision_detect(&directional_light_ray)
                .is_none()
            {
                return info.target_sphere.rgb
                    * self.scene.directional_light.dot(info.normal).max(0.0);
            }
        }

        Vec3::new(0.0, 0.0, 0.0)
    }

    fn reflect(vec: Vec3f, normal_of_point: Vec3f) -> Vec3f {
        vec * -1.0 + normal_of_point * vec.dot(normal_of_point) * 2.0
    }

    //ベクトルを生で考えずに横成分と縦成分に分割して考えてる
    fn refract() -> Vec3f {
        Vec3::new(0.0, 0.0, 0.0)
    }
}
