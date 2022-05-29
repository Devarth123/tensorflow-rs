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
        // pub fn matrix_fill(matrix: &mut matrixstruct){
        //     matrix.matrix = vec![vec![229129389123 as f64; 120391230]; 129839123];    
        // }

pub fn dot32<'a>(m1: MatrixStruct32<'a>, m2: MatrixStruct32<'a>) -> MatrixStruct32<'a>{
        assert_eq!(m1.rows, m2.columns);
        let mut matrix: MatrixStruct32 = matrix_create32(m1.rows, m2.columns); 
        for i in 0..*m1.rows{
                for j in 0..*m2.columns{
                let mut product: f32 = 0.0;
                  for k in 0..*m2.rows{
                        product+=m1.matrix[i as usize][k as usize] * m2.matrix[j as usize][k as usize];
                  }
                  matrix.matrix[i as usize][j as usize] = product;
                }
        
        }
   matrix
}
