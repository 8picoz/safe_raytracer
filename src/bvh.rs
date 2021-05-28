use std::ops::Range;
use std::usize;

use crate::intersect_info::IntersectInfo;
use crate::ray::Ray;
use crate::shapes::aabb::AABB;
use crate::shapes::obj::Obj;
use crate::shapes::sphere::Sphere;
use crate::shapes::triangle::Triangle;
use crate::shapes::Shapes;
use crate::vec3::Axis;
use crate::vec3::Vec3f;

const LEAF_THRESHOLD: usize = 4;

#[allow(clippy::upper_case_acronyms)]
pub struct BVH {
    //Shapesの実態の所有権はScene(BVH)が持つべき
    //IntersectInfoなどのライフタイムはScene(BVH)が消えたらそもそも存在できない
    shapes: Vec<Shapes>,
    pub directional_light: Vec3f,
    bvh_root_node: Option<BVHNode>,
}

impl BVH {
    pub fn new(directional_light: Vec3f) -> Self {
        BVH {
            shapes: Vec::new(),
            directional_light,
            bvh_root_node: None,
        }
    }

    //TODO: addをgenericに
    pub fn add_sphere(&mut self, sphere: Sphere) {
        self.shapes.push(Shapes::Sphere(sphere));
    }

    pub fn add_triangle(&mut self, triangle: Triangle) {
        self.shapes.push(Shapes::Triangle(triangle))
    }

    pub fn add_obj(&mut self, obj: Obj) {
        for shape in obj.triangles {
            self.shapes.push(shape);
        }
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

    pub fn build_bvh(&mut self) {
        self.bvh_root_node = Some(self.build_bvh_node(0..self.shapes.len()));
    }

    //sliceで範囲指定
    fn build_bvh_node(&mut self, shapes_range: Range<usize>) -> BVHNode {

        let shapes = &mut self.shapes[shapes_range.clone()];

        let bbox = shapes.iter().fold(AABB::new_min_bound(), |acc, i| acc.merge_aabb(i.calc_aabb()));

        let shapes_len = shapes.len();
        if shapes_len <= LEAF_THRESHOLD {
            return Self::create_leaf_node(bbox, shapes_range, shapes_len);
        }

        let split_axis = shapes.iter().fold(AABB::new_min_bound(), |acc, i| acc.merge_point(i.calc_aabb().center())).longest_axis();
        
        shapes.sort_by(|a, b| a.calc_aabb().center().get_axis_value(split_axis).partial_cmp(&b.calc_aabb().center().get_axis_value(split_axis)).unwrap() );
        
        let split_idx = shapes_len / 2;
        if split_idx == 0 || split_idx == shapes_len {
            return Self::create_leaf_node(bbox, shapes_range, shapes_len);
        }

        let start = shapes_range.start;
        let end = shapes_range.end;

        let child_0 = self.build_bvh_node(start..(start + split_idx));
        let child_1 = self.build_bvh_node((start + split_idx)..end);

        BVHNode::new(bbox, shapes_range, shapes_len, Some(split_axis), Box::new([Some(child_0), Some(child_1)]))
    }

    fn create_leaf_node(bbox: AABB<f32>, shapes_range: Range<usize>, shapes_len: usize) -> BVHNode {
        BVHNode::new(bbox, shapes_range, shapes_len, None, Box::new([None, None]))
    }
}

#[allow(clippy::upper_case_acronyms)]
struct BVHNode {
    aabb: AABB<f32>,
    shapes_range: Range<usize>,
    shapes_len: usize,
    split_axis: Option<Axis>,
    children: Box<[Option<BVHNode>; 2]>,
}

impl BVHNode {
    fn new(aabb: AABB<f32>, shapes_range: Range<usize>, shapes_len: usize, split_axis: Option<Axis>, children: Box<[Option<BVHNode>; 2]>) -> Self {
        Self { aabb, shapes_range, shapes_len, split_axis, children }
    }
}