use super::*;

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