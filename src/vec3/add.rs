use super::*;

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