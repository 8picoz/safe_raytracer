use super::*;

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