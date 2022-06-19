use crate::ml::f64::matrix::MatrixStruct;

pub fn exp(double: &f64) -> f64 {
    double * std::f64::consts::E //2.71828
}

pub fn sigmoid(input: &f64) -> f64 {
    1.0 / (1.0 + exp(&(-1.0 * input)))
}

pub fn max(f: &f64) -> f64 {
    if *f > 0.0 {
        *f
    } else {
        0.0
    }
}

//0 means sigmoid and 1 means Relu
pub fn sigmoid_prime(m: &MatrixStruct) -> MatrixStruct {
    let mut matrix = MatrixStruct::from(&m.rows, &m.columns);
    {
        let subtracted_matrix = MatrixStruct::subtract(&matrix, m);
        matrix = MatrixStruct::subtract(m, &subtracted_matrix);
    }
    matrix
}

pub fn apply(function: &u8, m: &MatrixStruct) -> MatrixStruct {
    let mut matrix = MatrixStruct::from(&m.rows, &m.columns);
    if *function == 0 {
        for i in 0..m.rows as usize {
            for j in 0..m.columns as usize {
                matrix.matrix[i][j] = sigmoid(&(m.matrix[i][j]));
            }
        }
    } else if *function == 1 {
        for i in 0..matrix.rows {
            for j in 0..matrix.columns {
                matrix.matrix[i][j] = max(&m.matrix[i][j]);
            }
        }
    }
    matrix
}
