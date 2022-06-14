use crate::ml::f64::matrix;

pub struct Img {
    pub label: u8,
    pub matrix: matrix::MatrixStruct,
}

impl Img{
    pub fn new() -> Img{
        Img{
            label: 0,
            matrix: matrix::matrix_create(&28, &28)
        }
    }
}
