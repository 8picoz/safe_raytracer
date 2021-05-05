use crate::intersect_info::IntersectInfo;
use crate::ray::Ray;
use crate::shapes::obj::Obj;
use crate::shapes::rectangle::Rectangle;
use crate::shapes::sphere::Sphere;
use crate::shapes::Shapes;
use crate::shapes::triangle::Triangle;
use crate::vec3::Vec3f;

pub struct Scene {
    //Shapesの実態の所有権はSceneが持つべき
    //IntersectInfoなどのライフタイムはSceneが消えたらそもそも存在できない
    shapes: Vec<Shapes>,
    pub directional_light: Vec3f,
}

impl Scene {
    pub fn new(directional_light: Vec3f) -> Self {
        Scene {
            shapes: Vec::new(),
            directional_light,
        }
    }

    //TODO: addをgenericに
    pub fn add_sphere(&mut self, sphere: Sphere) {
        self.shapes.push(Shapes::Sphere(sphere));
    }

    pub fn add_triangle(&mut self, triangle: Triangle) {
        self.shapes.push(Shapes::Triangle(triangle))
    }

    pub fn add_rectangle(&mut self, rect: Rectangle) {
        self.shapes.push(Shapes::Rectangle(rect));
    }

    pub fn add_obj(&mut self, obj: Obj) {
        self.shapes.push(Shapes::Obj(obj))
    }

    pub fn collision_detect(&self, ray: &Ray) -> Option<IntersectInfo> {
        let mut infos = Vec::new();

        for shape in self.shapes.iter() {
            if let Some(info) = shape.collision_detect(ray) {
                infos.push(info);
            }
        }

        if infos.is_empty() {
            return None;
        }

        Some(
            infos
                .into_iter()
                .min_by(|a, b| {
                    a.distance
                        .partial_cmp(&b.distance)
                        .expect("failed to compare")
                })
                .expect("failed to pick max value"),
        )
    }
}
