use crate::ray::*;
use crate::vec3::*;

pub struct PinholeCamera {
    position: Vec3f,
    forward_direction: Vec3f,
    right_direction: Vec3f,
    up_direction: Vec3f,
    distance_to_pinhole: f32,
}

impl PinholeCamera {
    pub fn new(position: Vec3f, forward_direction: Vec3f, distance_to_pinhole: f32) -> Self {
        let (right_direction, up_direction) = forward_direction.make_basis();

        PinholeCamera {
            position,
            forward_direction,
            right_direction,
            up_direction,
            distance_to_pinhole,
        }
    }

    //(i, j) <> (u, v)
    pub fn make_ray_to_pinhole(&self, u: f32, v: f32) -> Ray {
        let origin = self.position + (self.right_direction * u) + (self.up_direction * v);
        let direction = (self.pinhole_position() - origin).normalized();

        Ray::new(origin, direction)
    }

    fn pinhole_position(&self) -> Vec3f {
        self.position + self.forward_direction * self.distance_to_pinhole
    }
}
