use crate::intersect_info::IntersectInfo;
use crate::material::Material;
use crate::ray::*;
use crate::vec3::*;

use self::sphere::Sphere;
pub mod sphere;

use self::rectangle::Rectangle;
pub mod rectangle;

//できるだけ動的ディスパッチをしないようにするため
pub enum Shapes {
    Sphere(Sphere),
    Rectangle(Rectangle),
}

impl Shapes {
    pub fn collision_detect(&self, ray: &Ray) -> Option<IntersectInfo> {
        match self {
            Shapes::Sphere(sphere) => {
                let c_to_o = ray.origin - sphere.center_position;
                let b = ray.direction.dot(c_to_o);
                let c = c_to_o.sqr_magnitude() - num::pow(sphere.radius, 2);
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
                    (hit_position - sphere.center_position).normalized(),
                    self,
                ))
            }
            _ => None,
        }
    }

    // TODO: トレイトとしてまとめる
    pub fn get_center_position(&self) -> Vec3f {
        match self {
            Shapes::Sphere(sphere) => sphere.center_position,
            _ => Vec3::from(0.0),
        }
    }

    pub fn get_material(&self) -> Material {
        match self {
            Shapes::Sphere(sphere) => sphere.material,
            _ => Material::Diffuce,
        }
    }

    pub fn get_kd(&self) -> Color {
        match self {
            Shapes::Sphere(sphere) => sphere.kd,
            _ => Vec3::from(0.0),
        }
    }
}
