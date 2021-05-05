use crate::intersect_info::IntersectInfo;
use crate::material::Material;
use crate::ray::{Ray, TMAX, TMIN};
use crate::vec3::{Color, Vec3f};

use self::sphere::Sphere;
pub mod sphere;

use self::triangle::Triangle;
pub mod triangle;

use self::rectangle::Rectangle;
pub mod rectangle;

use self::obj::Obj;
pub mod obj;

//できるだけ動的ディスパッチをしないようにするため
pub enum Shapes {
    Sphere(Sphere),
    Triangle(Triangle),
    Rectangle(Rectangle),
    Obj(Obj),
}

const K_EPSILON: f32 = 1e-6;

impl Shapes {
    //collisiotn_detectはそれぞれが持つべき(?)
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
            Shapes::Triangle(triangle) => {
                let e2 = triangle.v2 - triangle.v0;
                let alpha = ray.direction.cross(e2);
                let e1 = triangle.v1 - triangle.v0;
                let det = alpha.dot(e1);
                
                if det < K_EPSILON && -K_EPSILON < det {
                    return None;
                }

                let inv_det = 1. / det;
                let r = ray.origin - triangle.v0;
                
                let u = inv_det * alpha.dot(r);
                if !(0. ..=1.).contains(&u) {
                    return None;
                }

                let beta = r.cross(e1);

                let v = inv_det * ray.direction.dot(beta);
                if v < 0. || u + v > 1. {
                    return None;
                }

                let ans = inv_det * beta.dot(e2);

                if ans < 0. || ans < TMIN || ans > TMAX {
                    return None;
                }

                let hit_position = ray.point_on_ray(ans);

                Some(IntersectInfo::new(ans, hit_position, (triangle.v0.cross(triangle.v1)).normalized(), self))
            }
            Shapes::Rectangle(rect) => {
                let o_to_c = rect.get_center_position() - ray.origin;
                let norm = (rect.ru - rect.lu).cross(rect.ld - rect.lu).normalized();
                let d_n = ray.direction.dot(norm);

                let ans = o_to_c.dot(norm) / d_n;

                if ans < TMIN || TMAX < ans || !ans.is_finite() {
                    return None;
                }

                let hit_position = ray.point_on_ray(ans);

                if !rect.point_is_inside(hit_position) {
                    return None;
                }

                Some(IntersectInfo::new(ans, hit_position, norm, self))
            }
            Shapes::Obj(obj) => {
                //BVH
                None
            }
        }
    }

    // TODO: トレイトとしてまとめる
    //SphereやReactangleに対してShapeのようなトレイトを用意
    pub fn get_center_position(&mut self) -> Vec3f {
        match self {
            Shapes::Sphere(sphere) => sphere.center_position,
            Shapes::Triangle(triangle) => triangle.get_center_position(),
            Shapes::Rectangle(rect) => rect.get_center_position(),
            Shapes::Obj(obj) => obj.center_position,
        }
    }

    pub fn get_material(&self) -> Material {
        match self {
            Shapes::Sphere(sphere) => sphere.material,
            Shapes::Triangle(triangle) => triangle.material,
            Shapes::Rectangle(rect) => rect.material,
            Shapes::Obj(obj) => obj.material,
        }
    }

    pub fn get_kd(&self) -> Color {
        match self {
            Shapes::Sphere(sphere) => sphere.kd,
            Shapes::Triangle(triangle) => triangle.kd,
            Shapes::Rectangle(rect) => rect.kd,
            Shapes::Obj(obj) => obj.kd,
        }
    }
}
