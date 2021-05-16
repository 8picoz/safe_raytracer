use core::f32;
use std::f32::consts::PI;

use rand::Rng;
use rand::prelude::ThreadRng;

use crate::Raytracer;
use crate::vec3::Vec3f;

#[derive(Debug, Clone)]
pub struct Lambert {
    rho: Vec3f,
}

impl Lambert {
    pub fn new(rho: Vec3f) -> Self {
        Lambert { rho }
    }

    pub fn sample(&self, rng: &mut ThreadRng) -> (Vec3f, Vec3f, f32) {
        let (target_direction, pdf) = Raytracer::make_ray_direction_with_important_sampling(rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0));

        (self.rho / PI, target_direction, pdf)
    }
}