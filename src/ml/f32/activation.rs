use crate::ml::f32::matrix;

pub fn exp(double: &f32) -> f32{
    double * std::f32::consts::E //2.71828
}

pub fn sigmoid(input: &f32) -> f32{
    1.0 / (1.0 + exp(&(-1.0*input)))
}


pub fn sigmoid_prime(m: &matrix::MatrixStruct) -> matrix::MatrixStruct{
    let mut matrix = matrix::sigmoid_matrix(&m.rows, &m.columns);
    {
        let subtracted_matrix = matrix::subtract(&matrix, m);
        matrix = matrix::subtract(m, &subtracted_matrix);
    }
    matrix
}

// 0 means sigmoid and 1 relu (not added yet) 
pub fn apply(function: &u8, m: &matrix::MatrixStruct) -> matrix::MatrixStruct{
    let mut matrix = matrix::matrix_create(&m.rows, &m.columns);
   if *function == 0{
      for i in 0..m.rows as usize{
          for j in 0..m.columns as usize{
              matrix.matrix[i][j] = sigmoid(&(m.matrix[i][j]));
          }
      }  
   }  
   matrix
}
