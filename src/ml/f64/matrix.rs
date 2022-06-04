pub struct MatrixStruct{
        pub rows: u16,
        pub columns: u16,
        pub matrix: Vec<Vec<f64>>,
}
pub fn matrix_create<'a>(row: &'a u16, column: &'a u16) -> MatrixStruct{
        let vec: Vec<Vec<f64>> = vec![vec![0.0; *row as usize]; *column as usize];  
        MatrixStruct{
                rows: *row,
                columns: *column,
                matrix: vec,
        }
}
pub fn positive(f: &f64) -> f64{
        if *f<0.0{
                return *f * -1_f64;
        }
        *f
}
pub fn dot(m1: &MatrixStruct, m2: &MatrixStruct) ->  MatrixStruct{
        assert_eq!(m1.rows, m2.columns, "the dimensions dont match matrix1.row = {} \t matrix2.column = {}", m1.rows, m2.columns);
        let mut matrix: MatrixStruct = matrix_create(&m1.rows, &m2.columns); 
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
pub fn add(m1: &MatrixStruct, m2: &MatrixStruct) -> MatrixStruct{
        assert!(m1.rows == m2.rows && m1.columns == m2.columns, "the dimensions dont match matrix1.rows = {}, matrix1.columns = {}, matrix2.rows = {}, matrix2.columns = {}", m1.rows, m1.columns, m2.rows, m2.columns); 
        let mut matrix: MatrixStruct = matrix_create(&m1.rows, &m1.columns); 
        // if (m1.rows == m2.columns) && (m1.columns == m2.columns) {
                for i in 0..m1.rows as usize{
                        for j in 0..m1.columns as usize{
                                matrix.matrix[i][j] = m1.matrix[i][j] + m2.matrix[i][j];
                        }
                }
        matrix
}
pub fn subtract(m1: &MatrixStruct, m2: &MatrixStruct) -> MatrixStruct{
        assert!(m1.rows == m2.rows && m1.columns == m2.columns, "the dimensions dont match matrix1.rows = {}, matrix1.columns = {}, matrix2.rows = {}, matrix2.columns = {}", m1.rows, m1.columns, m2.rows, m2.columns); 
        let mut matrix = matrix_create(&m1.rows, &m1.columns);
        for i in 0..m1.rows as usize{
                for j in 0..m1.columns as usize{
                        matrix.matrix[i][j] = positive(&(m1.matrix[i][j] - m2.matrix[i][j]));
                }
        }
        matrix
}

pub fn multiply(m1: &MatrixStruct, m2: &MatrixStruct) -> MatrixStruct{
        assert!(m1.rows == m2.rows && m1.columns == m2.columns, "the dimensions dont match matrix1.rows = {}, matrix1.columns = {}, matrix2.rows = {}, matrix2.columns = {}", m1.rows, m1.columns, m2.rows, m2.columns); 
        let mut matrix = matrix_create(&m1.rows, &m1.columns);
        for i in 0..m1.rows as usize{
                for j in 0..m2.columns as usize {
                        matrix.matrix[i][j] = m1.matrix[i][j] * m2.matrix[i][j];
                }
        }
        matrix
}
pub fn add_by_scalar(m1: &MatrixStruct, num: &f64) -> MatrixStruct {
       let mut matrix = matrix_create(&m1.rows, &m1.columns);
       for i in 0..m1.rows as usize{
               for j in 0..m1.columns as usize{
                       matrix.matrix[i][j] += *num;
               }
       }
   matrix
}
pub fn multiply_by_scalar(m: &MatrixStruct, num: &f64) -> MatrixStruct{
        let mut matrix = matrix_create(&m.rows, &m.columns);
        for i in 0..m.rows as usize{
                for j in 0..m.columns as usize{
                        matrix.matrix[i][j] = m.matrix[i][j] * *num;

                }
        }
        matrix
}
pub fn transpose(m: &MatrixStruct) -> MatrixStruct{
        let mut matrix = matrix_create(&m.columns, &m.rows);
        for i in 0..m.rows as usize{
                for j in 0..m.columns as usize{
                        matrix.matrix[j][i] = m.matrix[i][j];
                }
        }
        matrix
}
pub fn copy_matrix(m: &MatrixStruct) -> MatrixStruct{
        let mut cp_matrix = matrix_create(&m.rows, &m.columns);
        for i in 0..m.rows as usize{
                for  j in 0..m.columns as usize{
                        cp_matrix.matrix[i][j] = m.matrix[i][j];
                }
        }
        cp_matrix
}
//sometimes the rust is not in the mood
pub fn scale(num:  &f64, m1: &MatrixStruct) -> MatrixStruct{
        let mut m = copy_matrix(m1);
        for i in 0..m.rows as usize{
                for j in 0..m.columns as usize{
                        m.matrix[i][j] *= num;
                }
        }
        m
} 
pub fn sigmoid_matrix<'a>(r: &'a u16, c: &'a u16) -> MatrixStruct{
        MatrixStruct{
                rows: *r,
                columns: *c,
                matrix: vec![vec![1.0; *r as usize]; *c as usize]
        } 
}