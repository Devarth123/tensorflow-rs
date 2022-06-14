use crate::ml::f64::matrix::MatrixStruct;

pub struct Img {
    pub label: u8,
    pub matrix: MatrixStruct,
}

impl Img{
    pub fn new() -> Img{
        Img{
            label: 0,
            matrix: MatrixStruct::matrix_create(&28, &28)
        }
    }
}
