use rand::thread_rng;
use rand::Rng;

use crate::intersect_info::IntersectInfo;
use crate::ray::Ray;
use crate::scene::Scene;
use crate::vec3::{Vec3, Vec3f};

pub struct RTAO {
    ao_sample: u32,
    max_distance: f32,
}

impl RTAO {
    pub fn new(ao_sample: u32, max_distance: f32) -> Self {
        RTAO {
            ao_sample,
            max_distance,
        }
    }

    pub fn rtao(&self, scene: &Scene, info: &IntersectInfo) -> f32 {
        let (v2, v3) = info.normal.make_basis();

        let mut rng = thread_rng();

        (0..self.ao_sample)
            .filter(|_| {
                let ray = Ray::new(
                    info.point,
                    RTAO::local_to_world(
                        RTAO::make_ray_direction(rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0)),
                        v2,
                        info.normal,
                        v3,
                    ),
                );
                if let Some(ao_info) = scene.collision_detect(&ray) {
                    return ao_info.distance < self.max_distance;
                }

                false
            })
            .count() as f32
            / self.ao_sample as f32
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
