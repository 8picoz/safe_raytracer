pub mod image;
pub mod material;
pub mod renderer;
pub mod pinhole_camera;
pub mod vec3;
pub mod matrix;
pub(crate) mod constant;

#[cfg(test)]
mod tests;

use core::f32;

use rand::prelude::ThreadRng;
use rand::Rng;

pub fn gamma(k: f32) -> f32 {
    k.powf(1.0 / 2.2)
}

fn russian_roulette(p: f32, rng: &mut ThreadRng) -> bool {
    let u = rng.gen_range(0.0..=1.0);

    u > p
}
