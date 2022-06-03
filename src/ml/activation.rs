use crate::ml::matrix;

pub fn exp(double: &f64) -> f64{
    double * std::f64::consts::E //2.71828
}

pub fn sigmoid(input: &f64) -> f64{
    1.0 / (1.0 + exp(&(-1.0*input)))
}


pub fn sigmoid_prime64<'a>(m: &'a matrix::MatrixStruct64) -> matrix::MatrixStruct64<'a>{
    let mut matrix = matrix::sigmoid_matrix(m.rows, m.columns);
    {
        let subtracted_matrix = matrix::subtract64(&matrix, m);
        matrix = matrix::subtract64(m, &subtracted_matrix);
    }
    matrix
}

// 0 means sigmoid and 1 relu (not added yet) 
pub fn apply(function: &u8, m: &mut matrix::MatrixStruct64) {
   if *function == 0{
      for i in 0..*m.rows as usize{
          for j in 0..*m.columns as usize{
              m.matrix[i][j] = sigmoid(&(m.matrix[i][j]));
          }
      }  
   }   
}
