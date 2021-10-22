pub mod intersect_info;
pub mod ray;
pub mod rtao;
pub mod bvh;
pub mod shapes;
pub(crate) mod sampling;

use std::rc::Rc;
use std::u32;

use ray::Ray;
use bvh::BVH;
use shapes::bsdf::BSDF;
use crate::vec3::{Color, Vec3, Vec3f};

use rand::prelude::ThreadRng;
use rand::thread_rng;

use crate::russian_roulette;

pub struct Raytracer<'a> {
    max_depth: u32,
    scene: &'a BVH,
}

impl<'a> Raytracer<'a> {
    pub fn new(max_depth: u32, scene: &'a BVH) -> Self {
        Raytracer {
            max_depth,
            scene,
        }
    }
    
    pub fn pathtrace(&self, ray: Ray, index: u32, p: f32, sample: u32) -> Color {
        let mut result: Vec3f = Vec3f::from(0.0);
        let mut rng = thread_rng();
        let ray = Rc::new(ray);
        for _ in 0..sample {
            result = result + self.trace(ray.clone(), index, p, &mut rng, Vec3f::from(1.0));
        }

        result / sample as f32
    }

    fn trace(&self, ray: Rc<Ray>, index: u32, p: f32, rng: &mut ThreadRng, throughput: Vec3f) -> Color {
        if self.max_depth <= index {
            return Vec3::from(0.0);
        }
        
        if russian_roulette(p, rng) {
            return Vec3::from(0.0);
        }

        let throughput = throughput / p;
    
        let ret_info = self.scene.collision_detect(&ray).unwrap();

        if let Some(info) = ret_info {
            let normal = info.normal;

            let (v2, v3) = normal.make_basis();

            let (bsdf, target_direction, pdf) = match info.target.get_bsdf() {
                BSDF::Lambert(lambert) => lambert.sample(rng),
            };

            let direction = sampling::local_to_world(
                target_direction,
                v2,
                normal,
                v3,
            );

            let cos = direction.dot(normal).max(0.0);
            
            let throughput = throughput * (bsdf * cos / pdf);

            return self.trace(
                Rc::new(Ray::new(info.point, direction)),
                index + 1,
                p,
                rng,
                throughput,
            );
        }

        Vec3f::from(1.0) * throughput
    }

    #[allow(dead_code)]
    fn reflect(vec: Vec3f, normal_of_point: Vec3f) -> Vec3f {
        vec * -1.0 + normal_of_point * vec.dot(normal_of_point) * 2.0
    }

    //ベクトルを生で考えずに横成分と縦成分に分割して考えてる
    //https://i.imgur.com/vD5gz5h.png
    #[allow(dead_code)]
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
}
