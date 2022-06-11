use crate::ml::f64::img;
use crate::ml::f64::matrix;
use std::fs;

// pub fn breaker(not_formated_string: &str) -> Vec<String> {
//     // print!("{not_formated_string}");
//     let mut formated_string: Vec<String> = Vec::new();
//     let mut push_str = String::new();
//     for i in not_formated_string.chars() {
//         if i == '\n' {
//             formated_string.push(push_str.clone());
//         } else {
//             push_str.push(i);
//         }
//     }
//     formated_string
// }
//
pub fn count_new_lines(string: &str) -> u16 {
    let mut new_lines = 0;
    for i in string.chars() {
        if i == '\n' {
            new_lines += 1;
        }
    }
    new_lines
}
// pub fn matrix_fit(unformated_data: &Vec<f64>) -> matrix::MatrixStruct{
//
// }
// pub fn matrix_fit(unformated_data: &Vec<f64>) -> matrix::MatrixStruct{
//     let mut matrix = matrix::matrix_create(&28, &28);
//     for i in 0..28{
//        let mut multiplier = 0;
//        for j in 0..28{
//            matrix.matrix[i][j] = unformated_data[j+multiplier];
//        }
//        multiplier += 28;
//     }
//     matrix
// }
// pub fn read_csv(path_of_the_file: &str, how_many_lines_do_want: &u8) -> Vec<Vec<f64>> {
//     let data = breaker(&fs::read_to_string(&path_of_the_file).unwrap());
//     let mut counter = 0;
//     let mut str_to_f32: Vec<Vec<f64>> = Vec::new();
//     for i in &data {
//         let mut push_vec: Vec<f64> = vec![0.0; 28];
//         for j in i.chars() {
//             match j {
//                 j if j != ',' => push_vec.push(
//                     j.to_string()
//                         .parse::<f64>()
//                         .expect("the method at line 64 is not working as expected"),
//                 ),
//                 '\n' => counter += 1,
//                 _ => (),
//             }
//             if counter == *how_many_lines_do_want {
//                 break;
//             }
//         }
//         str_to_f32.push(push_vec);
//     }
//     str_to_f32
// }
pub fn  line(line_no: &u16, lines: &std::str::Lines) -> String{
    let mut str = "";
    for i in 0..*line_no{
        if i == line_no -1 {
            str = lines.next().unwrap();
        }else {
            lines.next();
        }
    } 
    str.to_string()
}
pub fn read_csv(path_of_the_file: &str, lines: &u8) -> matrix::MatrixStruct {
    let imgs: Vec<Vec<img::Img>> = Vec::with_capacity(*lines as usize);
    static MAXCHAR: u16 = 10000;
    let lines: String = (fs::read_to_string(path_of_the_file).unwrap()).lines();
    let mut i: u16  = 0;
    while i < *lines {
        let j = 0;
        x
        i += 1;
    }
}
