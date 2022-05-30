pub struct MatrixStruct32<'a>{
            pub rows: &'a u16,
            pub columns: &'a u16,
            pub matrix: Vec<Vec<f32>>,
     }
pub struct MatrixStruct64<'a>{
            pub rows: &'a u16,
            pub columns: &'a u16,
            pub matrix: Vec<Vec<f64>>,
     }
pub fn matrix_create32<'a>(row: &'a u16, column: &'a u16) -> MatrixStruct32<'a>{
            let mut vec: Vec<Vec<f32>> = vec![vec![0.0; *row as usize]; *column as usize];  
            let mut product: MatrixStruct32 = MatrixStruct32{
                rows: row,
                columns: column,
                matrix: vec,
            };
           product
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
        // pub fn matrix_fill(matrix: &mut matrixstruct){
        //     matrix.matrix = vec![vec![229129389123 as f64; 120391230]; 129839123];    
        // }
pub fn matrix_create64<'a>(row: &'a u16, column: &'a u16) -> MatrixStruct64<'a>{
            let mut vec: Vec<Vec<f64>> = vec![vec![0.0; *row as usize]; *column as usize];  
            let mut product: MatrixStruct64 = MatrixStruct64{
                rows: row,
                columns: column,
                matrix: vec,
            };
           product
     }
pub fn dot32<'a>(m1: MatrixStruct32<'a>, m2: MatrixStruct32<'a>) ->  MatrixStruct32<'a>{
        assert_eq!(m1.rows, m2.columns, "the dimensions dont match matrix1.row = {} \t matrix2.column = {}", m1.rows, m2.columns);
        let mut matrix: MatrixStruct32 = matrix_create32(m1.rows, m2.columns); 
        for i in 0..*m1.rows as usize{
                for j in 0..*m2.columns as usize{
                let mut product: f32 = 0.0;
                  for k in 0..*m2.rows as usize{
                        product+=m1.matrix[i][k] * m2.matrix[j][k];
                  }
                  matrix.matrix[i][j] = product;
                }
        
        }
   matrix
}

pub fn dot64<'a>(m1: MatrixStruct64<'a>, m2: MatrixStruct64<'a>) ->  MatrixStruct64<'a>{
        assert_eq!(m1.rows, m2.columns, "the dimensions dont match matrix1.row = {} \t matrix2.column = {}", m1.rows, m2.columns);
        // assert_eq!(m1.rows, m2.columns);
        let mut matrix: MatrixStruct64 = matrix_create64(m1.rows, m2.columns); 
        for i in 0..*m1.rows as usize{
                for j in 0..*m2.columns as usize{
                let mut product: f64 = 0.0;
                  for k in 0..*m2.rows as usize{
                        product+=m1.matrix[i][k] * m2.matrix[j][k];
                  }
                  matrix.matrix[i][j] = product;
                }
        
        }
   matrix
}
pub fn add32<'a>(m1: &'a MatrixStruct32, m2: &MatrixStruct32) -> MatrixStruct32<'a>{
        assert!(m1.rows == m2.rows && m1.columns == m2.columns, "the dimensions dont match matrix1.rows = {}, matrix1.columns = {}, matrix2.rows = {}, matrix2.columns = {}", m1.rows, m1.columns, m2.rows, m2.columns); 
        let mut matrix: MatrixStruct32 = matrix_create32(m1.rows, m1.columns); 
        // if (m1.rows == m2.columns) && (m1.columns == m2.columns) {
                for i in 0..*m1.rows as usize{
                        for j in 0..*m1.columns as usize{
                                matrix.matrix[i][j] = m1.matrix[i][j] + m2.matrix[i][j];
                        }
                }
        matrix
}
pub fn add64<'a>(m1: &'a MatrixStruct64, m2: &MatrixStruct64) -> MatrixStruct64<'a>{
        assert!(m1.rows == m2.rows && m1.columns == m2.columns, "the dimensions dont match matrix1.rows = {}, matrix1.columns = {}, matrix2.rows = {}, matrix2.columns = {}", m1.rows, m1.columns, m2.rows, m2.columns); 
        let mut matrix: MatrixStruct64 = matrix_create64(m1.rows, m1.columns); 
                for i in 0..*m1.rows as usize{
                        for j in 0..*m1.columns as usize{
                                matrix.matrix[i][j] = m1.matrix[i][j] + m2.matrix[i][j];
                        }
                }
        matrix 
}
pub fn subtract32<'a>(m1: &'a MatrixStruct32, m2: &MatrixStruct32) -> MatrixStruct32<'a>{
        assert!(m1.rows == m2.rows && m1.columns == m2.columns, "the dimensions dont match matrix1.rows = {}, matrix1.columns = {}, matrix2.rows = {}, matrix2.columns = {}", m1.rows, m1.columns, m2.rows, m2.columns); 
        let mut matrix = matrix_create32(m1.rows, m1.columns);
        for i in 0..*m1.rows as usize{
                for j in 0..*m1.columns as usize{
                        matrix.matrix[i][j] = positive32(&(m1.matrix[i][j] - m2.matrix[i][j]));
                }
        }
        matrix
}

pub fn subtract64<'a>(m1: &'a MatrixStruct64, m2: &MatrixStruct64) -> MatrixStruct64<'a>{
        assert!(m1.rows == m2.rows && m1.columns == m2.columns, "the dimensions dont match matrix1.rows = {}, matrix1.columns = {}, matrix2.rows = {}, matrix2.columns = {}", m1.rows, m1.columns, m2.rows, m2.columns); 
        let mut matrix = matrix_create64(m1.rows, m1.columns);
        for i in 0..*m1.rows as usize{
                for j in 0..*m1.columns as usize{
                        matrix.matrix[i][j] = positive64(&(m1.matrix[i][j] - m2.matrix[i][j]));
                }
        }
        matrix
}
pub fn multiply32<'a>(m1: &'a MatrixStruct32, m2: &MatrixStruct32) -> MatrixStruct32<'a>{
        assert!(m1.rows == m2.rows && m1.columns == m2.columns, "the dimensions dont match matrix1.rows = {}, matrix1.columns = {}, matrix2.rows = {}, matrix2.columns = {}", m1.rows, m1.columns, m2.rows, m2.columns); 
        let mut matrix = matrix_create32(m1.rows, m1.columns);
        for i in 0..*m1.rows as usize{
                for j in 0..*m2.columns as usize {
                        matrix.matrix[i][j] = m1.matrix[i][j] * m2.matrix[i][j];
                }
        }
        matrix
}

pub fn multiply64<'a>(m1: &'a MatrixStruct64, m2: &MatrixStruct64) -> MatrixStruct64<'a>{
        assert!(m1.rows == m2.rows && m1.columns == m2.columns, "the dimensions dont match matrix1.rows = {}, matrix1.columns = {}, matrix2.rows = {}, matrix2.columns = {}", m1.rows, m1.columns, m2.rows, m2.columns); 
        let mut matrix = matrix_create64(m1.rows, m1.columns);
        for i in 0..*m1.rows as usize{
                for j in 0..*m2.columns as usize {
                        matrix.matrix[i][j] = m1.matrix[i][j] * m2.matrix[i][j];
                }
        }
        matrix
}
// pub fn 
