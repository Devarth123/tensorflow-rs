use crate::ml::matrix;


pub fn exp(double: &f64) -> f64{
    double * std::f64::consts::E //2.71828
}

pub fn sigmoid(input: &f64) -> f64{
    1.0 / (1.0 + exp(&(-1.0*input)))
}
// pub fn sigmoidPrime(matrix: &matrix::MatrixStruct32) {
//     
// } 
