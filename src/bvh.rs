use core::f32;
use core::panic;
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
use crate::vec3::Vec3;
use crate::vec3::Vec3f;

const LEAF_THRESHOLD: usize = 4;

#[allow(clippy::upper_case_acronyms)]
pub struct BVH {
    //Shapesの実態の所有権はScene(BVH)が持つべき
    //IntersectInfoなどのライフタイムはScene(BVH)が消えたらそもそも存在できない
    shapes: Vec<Shapes>,
    pub directional_light: Vec3f,
    bvh_root_node: Option<BVHNode>,
    pub stats: BVHNodeStatics,
}

impl BVH {
    pub fn new(directional_light: Vec3f) -> Self {
        BVH {
            shapes: Vec::new(),
            directional_light,
            bvh_root_node: None,
            stats: BVHNodeStatics::new(),
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

    pub fn collision_detect(&self, ray: &Ray) -> Result<Option<IntersectInfo>, &'static str> {

        let dir_inv = Vec3::new(1.0 / ray.direction.x, 1.0 / ray.direction.y, 1.0 / ray.direction.z);
        let dir_inv_sign = Vec3::new(Self::check_sign(dir_inv.x), Self::check_sign(dir_inv.y), Self::check_sign(dir_inv.z));

        let bvh_root_node = match &self.bvh_root_node {
            Some(node) => node,
            None => return Err("you have to make a root node before call this method"),
        };

        let mut t_max = ray.t_max;
        Ok(self.intersect_node(bvh_root_node, ray, dir_inv, dir_inv_sign, &mut t_max))
    }

    pub fn build_bvh(&mut self) {
        self.bvh_root_node = Some(self.build_bvh_node(0..self.shapes.len()));

        self.stats.n_nodes = self.stats.n_internal_nodes + self.stats.n_leaf_nodes;
    }

    //sliceで範囲指定
    fn build_bvh_node(&mut self, shapes_range: Range<usize>) -> BVHNode {

        let shapes = &mut self.shapes[shapes_range.clone()];

        let bbox = shapes.iter().fold(AABB::new(Vec3::from(f32::MAX), Vec3::from(f32::MIN)), |acc, i| acc.merge_aabb(i.calc_aabb()));

        let shapes_len = shapes.len();
        if shapes_len <= LEAF_THRESHOLD {
            return self.create_leaf_node(bbox, shapes_range);
        }

        let split_axis = shapes.iter().fold(AABB::new(Vec3::from(f32::MAX), Vec3::from(f32::MIN)), |acc, i| acc.merge_point(i.calc_aabb().center())).longest_axis();

        shapes.sort_by(|a, b| a.calc_aabb().center().get_axis_value(split_axis).partial_cmp(&b.calc_aabb().center().get_axis_value(split_axis)).unwrap() );
        
        let split_idx = shapes_len / 2;

        if split_idx == 0 || split_idx == shapes_len {
            println!("splitting failed");
            return self.create_leaf_node(bbox, shapes_range);
        }

        let start = shapes_range.start;
        let end = shapes_range.end;

        let child_0 = self.build_bvh_node(start..(start + split_idx));
        let child_1 = self.build_bvh_node((start + split_idx)..end);
        self.stats.n_internal_nodes += 1;

        BVHNode::new(bbox, shapes_range, Some(split_axis), Box::new([Some(child_0), Some(child_1)]))
    }

    fn create_leaf_node(&mut self, bbox: AABB<f32>, shapes_range: Range<usize>) -> BVHNode {
        self.stats.n_leaf_nodes += 1;
        BVHNode::new(bbox, shapes_range, None, Box::new([None, None]))
    }

    fn check_sign(value: f32) -> usize {
        if value > 0.0 {
            0
        } else {
            1
        }
    }

    fn intersect_node(&self, node: &BVHNode, ray: &Ray, dir_inv: Vec3<f32>, dir_inv_sign: Vec3<usize>, t_max: &mut f32) -> Option<IntersectInfo> {
        
        if node.aabb.intersect(ray, dir_inv, dir_inv_sign) {
            let mut ret_info = None;
            if node.children[0].is_none() && node.children[1].is_none() {
                //現在のノードが葉ノード
                for shape in &self.shapes[node.shapes_range.clone()] {
                    if let Some(info) = shape.collision_detect_with_t_max(ray, *t_max) {
                        *t_max = info.distance;
                        ret_info = Some(info);
                    }
                }
            } else {
                //次の子ノードと交差判定
                let child_idx = match node.split_axis {
                    Some(Axis::X) => dir_inv_sign.x,
                    Some(Axis::Y) => dir_inv_sign.y,
                    Some(Axis::Z) => dir_inv_sign.z,
                    None => panic!("There can't use leaf node here but It's leaf node"),
                };

                ret_info = match &node.children[child_idx] {
                    Some(child) => {
                        self.intersect_node(child, ray, dir_inv, dir_inv_sign, t_max)
                    }
                    None => {
                        panic!("There can't use leaf node here but It's leaf node");
                    }
                };

                ret_info = if let Some(info) = match &node.children[1 - child_idx] {
                    Some(child) => {
                        self.intersect_node(child, ray, dir_inv, dir_inv_sign, t_max)
                    }
                    None => {
                        panic!("There can't use leaf node here but It's leaf node");
                    }
                } { Some(info) } else { ret_info };
            }

            return ret_info;
        }

        None
    }
}

#[allow(clippy::upper_case_acronyms)]
struct BVHNode {
    aabb: AABB<f32>,
    shapes_range: Range<usize>,
    split_axis: Option<Axis>,
    children: Box<[Option<BVHNode>; 2]>,
}

impl BVHNode {
    fn new(aabb: AABB<f32>, shapes_range: Range<usize>, split_axis: Option<Axis>, children: Box<[Option<BVHNode>; 2]>) -> Self {
        Self { aabb, shapes_range, split_axis, children }
    }
}

#[allow(clippy::upper_case_acronyms)]
pub struct BVHNodeStatics {
    pub n_nodes: i32,
    pub n_internal_nodes: i32,
    pub n_leaf_nodes: i32,
}

impl BVHNodeStatics {
    fn new() -> Self {
        Self { n_nodes: 0, n_internal_nodes: 0, n_leaf_nodes: 0 }
    }
}