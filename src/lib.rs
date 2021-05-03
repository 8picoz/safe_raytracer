pub mod image;
pub mod intersect_info;
pub mod material;
pub mod pinhole_camera;
pub mod ray;
pub mod rtao;
pub mod scene;
pub mod shapes;
pub mod vec3;

#[cfg(test)]
mod tests;

use core::f32;
use std::f32::consts::PI;
use std::u32;

use material::Material;
use num::abs;
use rand::prelude::ThreadRng;
use rand::thread_rng;
use ray::Ray;
use rtao::RTAO;
use scene::Scene;
use vec3::{Vec3, Vec3f, Color};
use rand::Rng;

pub struct Raytracer<'a> {
    max_depth: u32,
    scene: &'a Scene,
    rho: f32,
}

impl<'a> Raytracer<'a> {
    pub fn new(max_depth: u32, scene: &'a Scene, rho: f32) -> Self {
        Raytracer { max_depth, scene, rho }
    }

    pub fn raytrace(&self, camera_ray: Ray, ao_sampling_point: u32, index: u32) -> Color {
        if self.max_depth <= index {
            return Vec3::new(0.0, 0.0, 0.0);
        }

        if let Some(info) = self.scene.collision_detect(&camera_ray) {
            match info.target.get_material() {
                Material::Mirror => {
                    return self.raytrace(
                        Ray::new(
                            info.point,
                            Raytracer::reflect(camera_ray.direction * -1.0, info.normal)
                                .normalized(),
                        ),
                        ao_sampling_point,
                        index + 1,
                    );
                }
                Material::Glass => {
                    let is_inside = (camera_ray.direction * -1.0).dot(info.normal) < 0.0;

                    if !is_inside {
                        //球体のガラスが内側からか外側からかで屈折率が変化する
                        //他にも法線の向きが逆に
                        if let Some(direction) =
                            Raytracer::refract(camera_ray.direction * -1.0, info.normal, 1.0, 1.5)
                        {
                            return self.raytrace(
                                Ray::new(info.point, direction.normalized()),
                                ao_sampling_point,
                                index + 1,
                            );
                        }

                        return Vec3::new(0.0, 0.0, 0.0);
                    }

                    if let Some(direction) = Raytracer::refract(
                        camera_ray.direction * -1.0,
                        info.normal * -1.0,
                        1.5,
                        1.0,
                    ) {
                        return self.raytrace(
                            Ray::new(info.point, direction.normalized()),
                            ao_sampling_point,
                            index + 1,
                        );
                    }

                    return Vec3::new(0.0, 0.0, 0.0);
                }
                _ => (),
            }

            let directional_light_ray = Ray::new(info.point, self.scene.directional_light);
            let ray_info = self.scene.collision_detect(&directional_light_ray);

            let rtao = RTAO::new(100, 10.0, 1.0);

            let kdao = info.target.get_kd() * 0.1 * (1.0 - rtao.rtao(self.scene, &info));

            if let Some(ray_info) = ray_info {
                if ray_info.target.get_material() == Material::Glass {
                    return info.target.get_kd()
                        * self.scene.directional_light.dot(info.normal).max(0.0)
                        + kdao;
                } else {
                    return kdao;
                }
            } else {
                return info.target.get_kd()
                    * self.scene.directional_light.dot(info.normal).max(0.0)
                    + kdao;
            } 
        }

        //空
        Vec3::new(1.0, 1.0, 1.0)
    }

    pub fn pathtrace(&self, ray: Ray, index: u32, p: f32, sample: u32) -> Color {
        
        let mut result: Vec3f = Vec3f::from(0.0);
        let mut rng = thread_rng();
        for _ in 0..sample {
            result = result + self.trace(ray.clone(), index, p, &mut rng, Vec3f::from(1.0));
        }

        result / sample as f32
    }

    fn trace(&self, ray: Ray, index: u32, p: f32, rng: &mut ThreadRng, throughput: Vec3f) -> Color  {
        let brdf = self.rho / PI;
        let pdf = 1. / (2. * PI);

        if self.max_depth <= index {
            return throughput;
        }

        if russian_roulette(p, rng) {
            return throughput;
        }
        let throughput = throughput / p;

        if let Some(info) = self.scene.collision_detect(&ray) {

            let (v2, v3) = info.normal.make_basis();

            let direction = Raytracer::local_to_world(
                    Raytracer::make_ray_direction(rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0)),
                    v2, 
                    info.normal, 
                    v3);

            let cos = abs(direction.dot(info.normal));

            let throughput = throughput * (brdf * cos / pdf);

            return self.trace(
                Ray::new(info.point, direction),
                index + 1,
                p,
                rng,
                throughput
            );
        }

        Vec3f::from(1.0) * throughput
    }

    fn reflect(vec: Vec3f, normal_of_point: Vec3f) -> Vec3f {
        vec * -1.0 + normal_of_point * vec.dot(normal_of_point) * 2.0
    }

    //ベクトルを生で考えずに横成分と縦成分に分割して考えてる
    //https://i.imgur.com/vD5gz5h.png
    fn refract(in_vec: Vec3f, normal_of_point: Vec3f, in_ior: f32, out_ior: f32) -> Option<Vec3f> {
        let cos1 = in_vec.dot(normal_of_point);
        let in_vech = in_vec - normal_of_point * cos1;

        let out_vech = in_vech * -(in_ior / out_ior);

        if out_vech.magnitude() > 1.0 {
            return None;
        }

        let cos2 = (1.0 - out_vech.sqr_magnitude()).sqrt();
        let out_vecp = normal_of_point * -cos2;

        Some(out_vech + out_vecp)
    }

    fn make_ray_direction(u: f32, v: f32) -> Vec3f {
        let theta = (1.0 - u).acos();
        let phi = 2.0 * std::f32::consts::PI * v;

        Vec3::new(phi.cos() * theta.sin(), 1.0 - u, phi.sin() * theta.sin())
    }

    fn local_to_world(direction: Vec3f, lx: Vec3f, ly: Vec3f, lz: Vec3f) -> Vec3f {
        lx * direction.x + ly * direction.y + lz * direction.z
    }
}

pub fn gamma(k: f32) -> f32 {
    k.powf(1.0 / 2.2)
}

fn russian_roulette(p: f32, rng: &mut ThreadRng) -> bool {
    let u = rng.gen_range(0.0..=1.0);

    u > p
}