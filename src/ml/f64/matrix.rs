
#[derive(Debug, Clone)]
pub struct MatrixStruct {
    pub rows: usize,
    pub columns: usize,
    pub matrix: Vec<Vec<f64>>,
}
impl MatrixStruct {
    pub fn from<'a>(row: &'a usize, column: &'a usize) -> MatrixStruct {
        let vec: Vec<Vec<f64>> = vec![vec![0.0; *row  ]; *column ];
        MatrixStruct {
            rows: *row,
            columns: *column,
            matrix: vec,
        }
    }
    pub fn matrix_display(&self) {
        for i in 0..self.rows  {
            for j in 0..self.columns   {
                print!("{} ", self.matrix[i][j]);
            }
            println!();
        }
    }
    pub fn positive(f: &f64) -> f64 {
        if *f < 0.0 {
            return *f * -1_f64;
        }
        *f
    }
    pub fn dot(m1: &MatrixStruct, m2: &MatrixStruct) -> MatrixStruct {
        assert_eq!(
            m1.rows, m2.columns,
            "the dimensions dont match matrix1.row = {} \t matrix2.column = {}",
            m1.rows, m2.columns
        );
        let mut matrix: MatrixStruct = MatrixStruct::from(&m1.rows, &m2.columns);
        for i in 0..m1.rows   {
            for j in 0..m2.columns   {
                let mut product: f64 = 0.0;
                for k in 0..m2.rows   {
                    product += m1.matrix[i][k] * m2.matrix[j][k];
                }
                matrix.matrix[i][j] = product;
            }
        }
        matrix
    }
    pub fn add(m1: &MatrixStruct, m2: &MatrixStruct) -> MatrixStruct {
        assert!(m1.rows == m2.rows && m1.columns == m2.columns, "the dimensions dont match matrix1.rows = {}, matrix1.columns = {}, matrix2.rows = {}, matrix2.columns = {}", m1.rows, m1.columns, m2.rows, m2.columns);
        let mut matrix: MatrixStruct = MatrixStruct::from(&m1.rows, &m1.columns);
        // if (m1.rows == m2.columns) && (m1.columns == m2.columns) {
        for i in 0..m1.rows   {
            for j in 0..m1.columns   {
                matrix.matrix[i][j] = m1.matrix[i][j] + m2.matrix[i][j];
            }
        }
        matrix
    }
    pub fn subtract(m1: &MatrixStruct, m2: &MatrixStruct) -> MatrixStruct {
        assert!(m1.rows == m2.rows && m1.columns == m2.columns, "the dimensions dont match matrix1.rows = {}, matrix1.columns = {}, matrix2.rows = {}, matrix2.columns = {}", m1.rows, m1.columns, m2.rows, m2.columns);
        let mut matrix = MatrixStruct::from(&m1.rows, &m1.columns);
        for i in 0..m1.rows   {
            for j in 0..m1.columns   {
                matrix.matrix[i][j] = MatrixStruct::positive(&(m1.matrix[i][j] - m2.matrix[i][j]));
            }
        }
        matrix
    }

    pub fn multiply(m1: &MatrixStruct, m2: &MatrixStruct) -> MatrixStruct {
        assert!(m1.rows == m2.rows && m1.columns == m2.columns, "the dimensions dont match matrix1.rows = {}, matrix1.columns = {}, matrix2.rows = {}, matrix2.columns = {}", m1.rows, m1.columns, m2.rows, m2.columns);
        let mut matrix = MatrixStruct::from(&m1.rows, &m1.columns);
        for i in 0..m1.rows   {
            for j in 0..m2.columns   {
                matrix.matrix[i][j] = m1.matrix[i][j] * m2.matrix[i][j];
            }
        }
        matrix
    }
    pub fn add_by_scalar(m1: &MatrixStruct, num: &f64) -> MatrixStruct {
        let mut matrix = MatrixStruct::from(&m1.rows, &m1.columns);
        for i in 0..m1.rows   {
            for j in 0..m1.columns   {
                matrix.matrix[i][j] += *num;
            }
        }
        matrix
    }
    pub fn multiply_by_scalar(m: &MatrixStruct, num: &f64) -> MatrixStruct {
        let mut matrix = MatrixStruct::from(&m.rows, &m.columns);
        for i in 0..m.rows   {
            for j in 0..m.columns   {
                matrix.matrix[i][j] = m.matrix[i][j] * *num;
            }
        }
        matrix
    }
    pub fn transpose(m: &MatrixStruct) -> MatrixStruct {
        assert_ne!(m.rows, m.columns);
        let mut matrix = MatrixStruct::from(&m.columns, &m.rows);
        for i in 0..m.rows {
            for j in 0..m.columns {
                matrix.matrix[j][i] = m.matrix[i][j];
            }
        }
        matrix
    }
    //sometimes the rust is not in the mood
    pub fn scale(num: &f64, m1: &MatrixStruct) -> MatrixStruct {
        let mut m = m1.clone();
        for i in 0..m.rows {
            for j in 0..m.columns {
                m.matrix[i][j] *= num;
            }
        }
        m
    }

    pub fn flatten(matrix: &MatrixStruct) -> MatrixStruct {
        let c = matrix.rows * matrix.columns;
        let mut m = MatrixStruct::from(&c, &0);
        let mut v: Vec<f64> = Vec::with_capacity(c);
        for i in 0..matrix.rows {
            for j in 0..matrix.columns {
                v.push(matrix.matrix[i][j]);
            }
        }
        m.matrix = vec![v; 1];
        m
    }
}
