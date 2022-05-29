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

