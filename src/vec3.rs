pub mod cross;
pub mod add;
pub mod sub;
pub mod div;
pub mod mul;
pub mod axis;

use num::Float;
use num::abs;
use std::ops;
use std::usize;

use self::axis::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3<T>
where
    T: Copy + Send + Sync,
{
    pub x: T,
    pub y: T,
    pub z: T,
}

pub struct Vec3Iterator<'a, T> where T: Copy + Send + Sync {
    resource: [&'a T; 3],
    curr: u32,
    //_marker: marker::PhantomData<&'a T>,
}

impl<'a, T> Vec3Iterator<'a, T> where T: Copy + Send + Sync {
    pub fn new(resource: [&'a T; 3], curr: u32) -> Self {
        Self { resource, curr }
    }
}

impl<'a, T> Iterator for Vec3Iterator<'a, T> where T: Copy + Send + Sync {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {

        self.resource.get(self.curr as usize).copied()

    }
}

impl From<f32> for Vec3<f32> {
    fn from(input: f32) -> Self {
        Vec3::new(input, input, input)
    }
}

impl Vec3<f32> {
    pub fn make_basis(self) -> (Self, Self) {
        let v2;

        if abs(self.y) < 0.9 {
            v2 = self.cross(Vec3::new(0.0, 1.0, 0.0)).normalized();
        } else {
            v2 = self.cross(Vec3::new(0.0, 0.0, -1.0)).normalized();
        }

        (v2, v2.cross(self).normalized())
    }
}

impl<T> Vec3<T>
where
    T: Copy + Send + Sync,
{
    pub fn new(x: T, y: T, z: T) -> Self {
        Vec3 { x, y, z }
    }

    pub fn iter(&self) -> Vec3Iterator<T> {
        let resource = [&self.x, &self.y, &self.z];

        Vec3Iterator::new(resource, 0)
    }

    pub fn get_axis_value(&self, axis: Axis) -> T {
        match axis {
            Axis::X => self.x,
            Axis::Y => self.y,
            Axis::Z => self.z,
        }
    }
}

impl<T> Vec3<T>
where
    T: Copy + Send + Sync + PartialOrd,
{
    pub fn cmp_each_max_value(&self, other: Self) -> Self {
        let max_x = if self.x > other.x { self.x } else { other.x };
        let max_y = if self.y > other.y { self.y } else { other.y };
        let max_z = if self.z > other.z { self.z } else { other.z };

        Vec3::new(max_x, max_y, max_z)
    }

    pub fn cmp_each_min_value(&self, other: Self) -> Self {
        let min_x = if self.x < other.x { self.x } else { other.x };
        let min_y = if self.y < other.y { self.y } else { other.y };
        let min_z = if self.z < other.z { self.z } else { other.z };

        Vec3::new(min_x, min_y, min_z)
    }
}

impl<T> Vec3<T>
where
    T: ops::Add<Output = T> + ops::Mul<Output = T> + Copy + Send + Sync,
{
    //内積
    pub fn dot(self, rhs: Self) -> T {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    //ノルム^2
    pub fn sqr_magnitude(self) -> T {
        self.dot(self)
    }
}

impl<T> Vec3<T>
where
    T: ops::Add<Output = T> + ops::Mul<Output = T> + Copy + Send + Sync + Float,
{
    //[TODO] ノルム
    pub fn magnitude(self) -> T {
        self.dot(self).sqrt()
    }

    pub fn normalized(self) -> Self {
        self / self.magnitude()
    }
}

pub type Vec3f = Vec3<f32>;
pub type Color = Vec3<f32>;