use crate::intersect_info::*;
use crate::material::Material;
use crate::ray::*;
use crate::vec3::*;

use super::Shape;

#[derive(Debug)]
pub struct Sphere {
    center_position: Vec3f,
    radius: f32,
    material: Material,
    kd: Vec3f,
}

impl Sphere {
    pub fn new(center_position: Vec3f, radius: f32, material: Material, kd: Vec3f) -> Self {
        Sphere {
            center_position,
            radius,
            material,
            kd,
        }
    }
}

impl Shape for Sphere {
    type Output = Self;

    fn collision_detect(&self, ray: &Ray) -> Option<IntersectInfo<Self>> {
        let c_to_o = ray.origin - self.center_position;
        let b = ray.direction.dot(c_to_o);
        let c = c_to_o.sqr_magnitude() - num::pow(self.radius, 2);
        #[allow(non_snake_case)]
        let D = num::pow(b, 2) - c;

        if D < 0.0 {
            return None;
        }

        let mut ans = -b - D.sqrt();
        if ans < TMIN || TMAX < ans {
            ans = -b + D.sqrt();

            if ans < TMIN || TMAX < ans {
                return None;
            }
        }

        let hit_position = ray.point_on_ray(ans);

        Some(IntersectInfo::new(
            ans,
            hit_position,
            (hit_position - self.center_position).normalized(),
            self,
        ))
    }

    fn get_center_potision(&self) -> Vec3f {
        self.center_position
    }

    fn get_material(&self) -> Material {
        self.material
    }

    fn get_kd(&self) -> Color {
        self.kd
    }
}