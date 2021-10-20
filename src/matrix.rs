use std::ops;

//Even though I didn't specify associate type in ops::Add and ops::Mul, why rust don't display error?
pub trait Bound<T>: ops::Add<T, Output = T> + ops::Mul<T, Output = T> + Copy + Send + Sync {} 

pub struct Matrix2x2<T> 
where
    T: Bound<T>,
{
    pub x0y0: T,
    pub x1y0: T,
    pub x0y1: T,
    pub x1y1: T,
}

pub struct Matrix3x3<T> 
where
    T: Bound<T>,
{
    pub x0y0: T,
    pub x1y0: T,
    pub x2y0: T,
    pub x0y1: T,
    pub x1y1: T,
    pub x2y1: T,
    pub x0y2: T,
    pub x1y2: T,
    pub x2y2: T,
}

pub struct Matrix4x4<T>
where
    T: Bound<T>,
{
    pub x0y0: T,
    pub x1y0: T,
    pub x2y0: T,
    pub x3y0: T,
    pub x0y1: T,
    pub x1y1: T,
    pub x2y1: T,
    pub x3y1: T,
    pub x0y2: T,
    pub x1y2: T,
    pub x2y2: T,
    pub x3y2: T,
    pub x0y3: T,
    pub x1y3: T,
    pub x2y3: T,
    pub x3y3: T,
}

impl<T> Matrix2x2<T> 
where 
    T: Bound<T>,
{
    pub fn new(x0y0: T, x1y0: T, x0y1: T, x1y1: T) -> Self
    {
        Matrix2x2 { x0y0, x1y0, x0y1, x1y1 }
    }    
}

impl<T> ops::Add for Matrix2x2<T> 
where 
    T: Bound<T>,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Matrix2x2::new(self.x0y0 + other.x0y0, self.x1y0 + other.x1y0, 
                        self.x0y1 + other.x0y1, self.x1y1 + other.x1y1)
    }
}

impl<T> Matrix3x3<T> 
where
    T: Bound<T>,
{
    #[allow(clippy::too_many_arguments)]
    pub fn new(x0y0: T, x1y0: T, x2y0: T, x0y1: T, x1y1: T, x2y1: T, x0y2: T, x1y2: T, x2y2: T) -> Self
    {
        Matrix3x3 { x0y0, x1y0, x2y0, x0y1, x1y1, x2y1, x0y2, x1y2, x2y2 }
    }
}

impl<T> ops::Add for Matrix3x3<T> 
where
    T: Bound<T>,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Matrix3x3::new(self.x0y0 + other.x0y0, self.x1y0 + other.x1y0, self.x2y0 + other.x2y0,
                        self.x0y1 + other.x0y1, self.x1y1 + other.x1y1, self.x2y1 + other.x2y1,
                        self.x0y2 + other.x0y2, self.x1y2 + other.x1y2, self.x2y2 + other.x2y2)
    }
}

impl<T> Matrix4x4<T>
where
    T: Bound<T>,
{
    #[allow(clippy::too_many_arguments)]
    pub fn new(x0y0: T, x1y0: T, x2y0: T, x3y0: T, x0y1: T, x1y1: T, x2y1: T, x3y1: T, x0y2: T, x1y2: T, x2y2: T, x3y2: T, x0y3: T, x1y3: T, x2y3: T, x3y3: T) -> Self
    {
        Matrix4x4 { x0y0, x1y0, x2y0, x3y0, x0y1, x1y1, x2y1, x3y1, x0y2, x1y2, x2y2, x3y2, x0y3, x1y3, x2y3, x3y3 }
    }
}

impl<T> ops::Add for Matrix4x4<T> 
where 
    T: Bound<T>,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Matrix4x4::new(self.x0y0 + other.x0y0, self.x1y0 + other.x1y0, self.x2y0 + other.x2y0, self.x3y0 + other.x3y0,
                        self.x0y1 + other.x0y1, self.x1y1 + other.x1y1, self.x2y1 + other.x2y1, self.x3y1 + other.x3y1,
                        self.x0y2 + other.x0y2, self.x1y2 + other.x1y2, self.x2y2 + other.x2y2, self.x3y2 + other.x3y2,
                        self.x0y3 + other.x0y3, self.x1y3 + other.x1y3, self.x2y3 + other.x2y3, self.x3y3 + other.x3y3)
    }
}