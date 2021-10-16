pub struct Matrix2x2<T> {
    pub x0y0: T,
    pub x0y1: T,
    pub x1y0: T,
    pub x1y1: T,
}

pub struct Matrix3x3<T> {
    pub x0y0: T,
    pub x0y1: T,
    pub x0y2: T,
    pub x1y0: T,
    pub x1y1: T,
    pub x1y2: T,
    pub x2y0: T,
    pub x2y1: T,
    pub x2y2: T,
}

pub struct Matrix4x4<T> {
    pub x0y0: T,
    pub x0y1: T,
    pub x0y2: T,
    pub x0y3: T,
    pub x1y0: T,
    pub x1y1: T,
    pub x1y2: T,
    pub x1y3: T,
    pub x2y0: T,
    pub x2y1: T,
    pub x2y2: T,
    pub x2y3: T,
    pub x3y0: T,
    pub x3y1: T,
    pub x3y2: T,
    pub x3y3: T,
}

pub fn new_matrix2x2<T>(x0y0: T, x0y1: T, x1y0: T, x1y1: T) -> Matrix2x2<T> {
    Matrix2x2 { x0y0, x0y1, x1y0, x1y1 }
}

#[allow(clippy::too_many_arguments)]
pub fn new_matrix3x3<T>(x0y0: T, x0y1: T, x0y2: T, x1y0: T, x1y1: T, x1y2: T, x2y0: T, x2y1: T, x2y2: T) -> Matrix3x3<T> {
    Matrix3x3 { x0y0, x0y1, x0y2, x1y0, x1y1, x1y2, x2y0, x2y1, x2y2 }
}

#[allow(clippy::too_many_arguments)]
pub fn new_matrix4x4<T>(x0y0: T, x0y1: T, x0y2: T, x0y3: T, x1y0: T, x1y1: T, x1y2: T, x1y3: T, x2y0: T, x2y1: T, x2y2: T, x2y3: T, x3y0: T, x3y1: T, x3y2: T, x3y3: T) -> Matrix4x4<T> {
    Matrix4x4 { x0y0, x0y1, x0y2, x0y3, x1y0, x1y1, x1y2, x1y3, x2y0, x2y1, x2y2, x2y3, x3y0, x3y1, x3y2, x3y3 }
}
