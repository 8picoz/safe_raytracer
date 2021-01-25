use num::Float;
use std::ops;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3<T>
where
    T: Copy,
{
    x: T,
    y: T,
    z: T,
}

impl<T> Vec3<T>
where
    T: Copy,
{
    pub fn new(x: T, y: T, z: T) -> Self {
        Vec3 { x, y, z }
    }
}

impl<T> Vec3<T>
where
    T: ops::Add<Output = T> + ops::Mul<Output = T> + Copy,
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
    T: ops::Add<Output = T> + ops::Mul<Output = T> + Copy + Float,
{
    //[TODO] ノルム
    pub fn magnitude(self) -> T {
        self.dot(self).sqrt()
    }

    pub fn normalized(self) -> Self {
        let magnitude = self.magnitude();

        Self {
            x: magnitude * self.x,
            y: magnitude * self.y,
            z: magnitude * self.z,
        }
    }
}

impl<T> Vec3<T> where T: ops::Add<Output = T> + ops::Mul<Output = T> + Copy {}

//外積
impl<T> Vec3<T>
where
    T: ops::Sub<Output = T> + ops::Mul<Output = T> + Copy,
{
    //外積
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
    T: ops::Add<Output = T> + Copy,
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
    T: ops::Sub<Output = T> + Copy,
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

//スカラー倍 vec3 * k
impl<T> ops::Mul<T> for Vec3<T>
where
    T: ops::Mul<Output = T> + Copy,
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
    T: ops::Mul<Output = T> + Copy,
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
    T: ops::Mul<Output = T> + Copy,
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
