pub struct MatrixStruct32{
        pub rows: u16,
        pub columns:  u16,
        pub matrix: Vec<Vec<f32>>,
}
pub struct MatrixStruct64{
        pub rows: u16,
        pub columns: u16,
        pub matrix: Vec<Vec<f64>>,
}
pub fn matrix_create32<'a>(row: &'a u16, column: &'a u16) -> MatrixStruct32{
        let vec: Vec<Vec<f32>> = vec![vec![0.0; *row as usize]; *column as usize];  
        MatrixStruct32{
                rows: *row,
                columns: *column,
                matrix: vec,
        }
}
pub fn positive32(f: &f32) -> f32{
        if *f<0.0{
                return *f * -1_f32;
        }
        *f
}
pub fn positive64(f: &f64) -> f64{
        if *f<0.0{
                return *f * -1_f64;
        }
        *f
}
pub fn matrix_create64<'a>(row: &'a u16, column: &'a u16) -> MatrixStruct64{
        MatrixStruct64{
                rows: *row,
                columns: *column,
                matrix: vec![vec![0.0; *row as usize]; *column as usize]
            }
     }
pub fn dot32(m1: &MatrixStruct32, m2: &MatrixStruct32) ->  MatrixStruct32{
        assert_eq!(m1.rows, m2.columns, "the dimensions dont match matrix1.row = {} \t matrix2.column = {}", m1.rows, m2.columns);
        let mut matrix: MatrixStruct32 = matrix_create32(&m1.rows, &m2.columns); 
        for i in 0..m1.rows as usize{
                for j in 0..m2.columns as usize{
                let mut product: f32 = 0.0;
                  for k in 0..m2.rows as usize{
                        product+=m1.matrix[i][k] * m2.matrix[j][k];
                  }
                  matrix.matrix[i][j] = product;
                }
        
        }
   matrix
}

pub fn dot64(m1:  &MatrixStruct64, m2: &MatrixStruct64) ->  MatrixStruct64{
        assert_eq!(m1.rows, m2.columns, "the dimensions dont match matrix1.row = {} \t matrix2.column = {}", m1.rows, m2.columns);
        // assert_eq!(m1.rows, m2.columns);
        let mut matrix: MatrixStruct64 = matrix_create64(&m1.rows, &m2.columns); 
        for i in 0..m1.rows as usize{
                for j in 0..m2.columns as usize{
                let mut product: f64 = 0.0;
                  for k in 0..m2.rows as usize{
                        product+=m1.matrix[i][k] * m2.matrix[j][k];
                  }
                  matrix.matrix[i][j] = product;
                }
        
        }
   matrix
}
pub fn add32(m1: &MatrixStruct32, m2: &MatrixStruct32) -> MatrixStruct32{
        assert!(m1.rows == m2.rows && m1.columns == m2.columns, "the dimensions dont match matrix1.rows = {}, matrix1.columns = {}, matrix2.rows = {}, matrix2.columns = {}", m1.rows, m1.columns, m2.rows, m2.columns); 
        let mut matrix: MatrixStruct32 = matrix_create32(&m1.rows, &m1.columns); 
        // if (m1.rows == m2.columns) && (m1.columns == m2.columns) {
                for i in 0..m1.rows as usize{
                        for j in 0..m1.columns as usize{
                                matrix.matrix[i][j] = m1.matrix[i][j] + m2.matrix[i][j];
                        }
                }
        matrix
}
pub fn add64(m1: &MatrixStruct64, m2: &MatrixStruct64) -> MatrixStruct64{
        assert!(m1.rows == m2.rows && m1.columns == m2.columns, "the dimensions dont match matrix1.rows = {}, matrix1.columns = {}, matrix2.rows = {}, matrix2.columns = {}", m1.rows, m1.columns, m2.rows, m2.columns); 
        let mut matrix: MatrixStruct64 = matrix_create64(&m1.rows, &m1.columns); 
                for i in 0..m1.rows as usize{
                        for j in 0..m1.columns as usize{
                                matrix.matrix[i][j] = m1.matrix[i][j] + m2.matrix[i][j];
                        }
                }
        matrix 
}
pub fn subtract32(m1: &MatrixStruct32, m2: &MatrixStruct32) -> MatrixStruct32{
        assert!(m1.rows == m2.rows && m1.columns == m2.columns, "the dimensions dont match matrix1.rows = {}, matrix1.columns = {}, matrix2.rows = {}, matrix2.columns = {}", m1.rows, m1.columns, m2.rows, m2.columns); 
        let mut matrix = matrix_create32(&m1.rows, &m1.columns);
        for i in 0..m1.rows as usize{
                for j in 0..m1.columns as usize{
                        matrix.matrix[i][j] = positive32(&(m1.matrix[i][j] - m2.matrix[i][j]));
                }
        }
        matrix
}

pub fn subtract64(m1: &MatrixStruct64, m2: &MatrixStruct64) -> MatrixStruct64{
        assert!(m1.rows == m2.rows && m1.columns == m2.columns, "the dimensions dont match matrix1.rows = {}, matrix1.columns = {}, matrix2.rows = {}, matrix2.columns = {}", m1.rows, m1.columns, m2.rows, m2.columns); 
        let mut matrix = matrix_create64(&m1.rows, &m1.columns);
        for i in 0..m1.rows as usize{
                for j in 0..m1.columns as usize{
                        matrix.matrix[i][j] = positive64(&(m1.matrix[i][j] - m2.matrix[i][j]));
                }
        }
        matrix
}
pub fn multiply32(m1: &MatrixStruct32, m2: &MatrixStruct32) -> MatrixStruct32{
        assert!(m1.rows == m2.rows && m1.columns == m2.columns, "the dimensions dont match matrix1.rows = {}, matrix1.columns = {}, matrix2.rows = {}, matrix2.columns = {}", m1.rows, m1.columns, m2.rows, m2.columns); 
        let mut matrix = matrix_create32(&m1.rows, &m1.columns);
        for i in 0..m1.rows as usize{
                for j in 0..m2.columns as usize {
                        matrix.matrix[i][j] = m1.matrix[i][j] * m2.matrix[i][j];
                }
        }
        matrix
}

pub fn multiply64(m1: &MatrixStruct64, m2: &MatrixStruct64) -> MatrixStruct64{
        assert!(m1.rows == m2.rows && m1.columns == m2.columns, "the dimensions dont match matrix1.rows = {}, matrix1.columns = {}, matrix2.rows = {}, matrix2.columns = {}", m1.rows, m1.columns, m2.rows, m2.columns); 
        let mut matrix = matrix_create64(&m1.rows, &m1.columns);
        for i in 0..m1.rows as usize{
                for j in 0..m2.columns as usize {
                        matrix.matrix[i][j] = m1.matrix[i][j] * m2.matrix[i][j];
                }
        }
        matrix
}
pub fn add_by_scalar32(m1: &MatrixStruct32, num: &f32) -> MatrixStruct32 {
       let mut matrix = matrix_create32(&m1.rows, &m1.columns);
       for i in 0..m1.rows as usize{
               for j in 0..m1.columns as usize{
                       matrix.matrix[i][j] += *num;
               }
       }
   matrix
}
pub fn add_by_scalar64(m1: &MatrixStruct64, num: &f64) -> MatrixStruct64 {
       let mut matrix = matrix_create64(&m1.rows, &m1.columns);
       for i in 0..m1.rows as usize{
               for j in 0..m1.columns as usize{
                       matrix.matrix[i][j] += *num;
               }
       }
   matrix
}
pub fn multiply_by_scalar32(m: &MatrixStruct32, num: &f32) -> MatrixStruct32{
        let mut matrix = matrix_create32(&m.rows, &m.columns);
        for i in 0..m.rows as usize{
                for j in 0..m.columns as usize{
                        matrix.matrix[i][j] = m.matrix[i][j] * *num;

                }
        }
        matrix
}
pub fn multiply_by_scalar64(m: &MatrixStruct64, num: &f64) -> MatrixStruct64{
        let mut matrix = matrix_create64(&m.rows, &m.columns);
        for i in 0..m.rows as usize{
                for j in 0..m.columns as usize{
                        matrix.matrix[i][j] = m.matrix[i][j] * *num;

                }
        }
        matrix
}
pub fn transpose32(m: &MatrixStruct32) -> MatrixStruct32{
        let mut matrix = matrix_create32(&m.columns, &m.rows);
        for i in 0..m.rows as usize{
                for j in 0..m.columns as usize{
                        matrix.matrix[j][i] = m.matrix[i][j];
                }
        }
        matrix
}
pub fn transpose64(m: &MatrixStruct64) -> MatrixStruct64{
        let mut matrix = matrix_create64(&m.columns, &m.rows);
        for i in 0..m.rows as usize{
                for j in 0..m.columns as usize{
                        matrix.matrix[j][i] = m.matrix[i][j];
                }
        }
        matrix
}
pub fn copy_matrix(m: &MatrixStruct64) -> MatrixStruct64{
        let mut cp_matrix = matrix_create64(&m.rows, &m.columns);
        for i in 0..m.rows as usize{
                for  j in 0..m.columns as usize{
                        cp_matrix.matrix[i][j] = m.matrix[i][j];
                }
        }
        cp_matrix
}
//sometimes the rust is not in the mood
pub fn scale(num:  &f64, m1: &MatrixStruct64) -> MatrixStruct64{
        let mut m = copy_matrix(m1);
        for i in 0..m.rows as usize{
                for j in 0..m.columns as usize{
                        m.matrix[i][j] *= num;
                }
        }
        m
} 
pub fn sigmoid_matrix<'a>(r: &'a u16, c: &'a u16) -> MatrixStruct64{
        MatrixStruct64{
                rows: *r,
                columns: *c,
                matrix: vec![vec![1.0; *r as usize]; *c as usize]
        } 
}
