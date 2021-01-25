use std::ops;

#[derive(Debug)]
pub struct Vec3<T> {
    x: T,
    y: T,
    z: T,
}

impl<T> Vec3<T>
where 
    T: ops::Add,
    {
        pub fn new(x: T, y: T, z: T) -> Self {
            Vec3 { x, y, z }
        }
    }

impl<T> PartialEq for Vec3<T>
where 
    T: PartialEq,
    {
        fn eq(&self, rhs: &Self) -> bool {
            self.x == rhs.x && self.y == rhs.y && self.z == rhs.z
        }
    }

impl<T> ops::Add<Self> for Vec3<T>
where 
    T: ops::Add<Output = T>,
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

impl<T> ops::Sub<Self> for Vec3<T> 
where
    T: ops::Sub<Output = T>,
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