use crate::ml::f32::matrix::MatrixStruct;

pub fn exp(double: &f32) -> f32 {
    double * std::f32::consts::E //2.71828
}

pub fn sigmoid(input: &f32) -> f32 {
    1.0 / (1.0 + exp(&(-1.0 * input)))
}

pub fn sigmoid_prime(m: &MatrixStruct) -> MatrixStruct {
    let mut matrix = MatrixStruct::from(&m.rows, &m.columns);
    {
        let subtracted_matrix = MatrixStruct::subtract(&matrix, m);
        matrix = MatrixStruct::subtract(m, &subtracted_matrix);
    }
    matrix
}

// 0 means sigmoid and 1 relu (not added yet)
pub fn apply(function: &u8, m: &MatrixStruct) -> MatrixStruct {
    let mut matrix = MatrixStruct::from(&m.rows, &m.columns);
    if *function == 0 {
        for i in 0..m.rows {
            for j in 0..m.columns {
                matrix.matrix[i][j] = sigmoid(&(m.matrix[i][j]));
            }
        }
    }
    matrix
}
