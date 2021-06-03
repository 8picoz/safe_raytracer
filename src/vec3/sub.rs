use super::*;

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