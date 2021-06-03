use super::*;

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