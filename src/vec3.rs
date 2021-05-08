use num::Float;
use num::abs;
use std::ops;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3<T>
where
    T: Copy + Send + Sync,
{
    pub x: T,
    pub y: T,
    pub z: T,
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

//外積
impl<T> Vec3<T>
where
    T: ops::Sub<Output = T> + ops::Mul<Output = T> + Copy + Send + Sync,
{
    pub fn cross(self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

//加算
impl<T> ops::Add<Self> for Vec3<T>
where
    T: ops::Add<Output = T> + Copy + Send + Sync,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

//減算
impl<T> ops::Sub<Self> for Vec3<T>
where
    T: ops::Sub<Output = T> + Copy + Send + Sync,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

//除算 vec3 / k
impl<T> ops::Div<T> for Vec3<T>
where
    T: ops::Div<Output = T> + Copy + Send + Sync,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

//スカラー倍 vec3 * k
impl<T> ops::Mul<T> for Vec3<T>
where
    T: ops::Mul<Output = T> + Copy + Send + Sync,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

//ニュータイプ
pub struct MultiplableType<T: ops::Mul>(T);

//スカラー倍 k * vec3
impl<T> ops::Mul<Vec3<T>> for MultiplableType<T>
where
    T: ops::Mul<Output = T> + Copy + Send + Sync,
{
    type Output = Vec3<T>;

    fn mul(self, rhs: Vec3<T>) -> Vec3<T> {
        let Self(value) = self;

        Vec3 {
            x: value * rhs.x,
            y: value * rhs.y,
            z: value * rhs.z,
        }
    }
}

//アダマール積
impl<T> ops::Mul<Self> for Vec3<T>
where
    T: ops::Mul<Output = T> + Copy + Send + Sync,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

pub type Vec3f = Vec3<f32>;
pub type Color = Vec3<f32>;
