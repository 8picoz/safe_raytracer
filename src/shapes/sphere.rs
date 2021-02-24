use crate::intersect_info::*;
use crate::material::Material;
use crate::ray::*;
use crate::vec3::*;

#[derive(Debug)]
pub struct Sphere {
    pub point: Vec3f,
    pub radius: f32,
    pub material: Material,
    pub kd: Vec3f,
}

impl Sphere {
    pub fn new(point: Vec3f, radius: f32, material: Material, kd: Vec3f) -> Self {
        Sphere {
            point,
            radius,
            material,
            kd,
        }
    }

    pub fn collision_detect(&self, ray: &Ray) -> Option<IntersectInfo> {
        let c_to_o = ray.origin - self.point;
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
            (hit_position - self.point).normalized(),
            self,
        ))
    }
}
