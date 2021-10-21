use std::f32::consts::PI;

use crate::constant::INV_PI;
use crate::vec3::Vec3;
use crate::vec3::Vec3f;

pub fn make_ray_direction_with_important_sampling(u: f32, v: f32) -> (Vec3f, f32) {
    let theta = (1. / 2.) * (1. - 2. * u).clamp(-1.0, 1.0).acos();
    let phi = 2.0 * PI * v;

    let pdf: f32 = theta.cos() * INV_PI;

    (Vec3::new(phi.cos() * theta.sin(), theta.cos(), phi.sin() * theta.sin()), pdf)
}

pub fn local_to_world(direction: Vec3f, lx: Vec3f, ly: Vec3f, lz: Vec3f) -> Vec3f {
    lx * direction.x + ly * direction.y + lz * direction.z
}