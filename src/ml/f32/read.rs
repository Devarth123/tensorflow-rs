use crate::ml::f32::img;
use std::fs::File;
use std::io::prelude::Read;

pub fn count_new_lines(string: &str) -> u16 {
    let mut new_lines = 0;
    for i in string.chars() {
        if i == '\n' {
            new_lines += 1;
        }
    }
    new_lines
}

pub fn format_data<const LEN: usize>(string: &[u8], lines: &usize) -> Vec<img::Img> {
    let mut imgs: Vec<img::Img> = Vec::with_capacity(*lines);
    let mut a: [f32; LEN] = [0.0; LEN];
    let iter = 0;
    for i in 0..string.len() {
        if string[i] != b',' {
            if string[i] != b'\n' {
                a[i] = string[i].to_string().parse::<f32>().unwrap();
            } else {
                let mut img = img::Img::new();
                for j in 0..28 {
                    if j == 0 {
                        img.label = a[j] as u8;
                    } else {
                        img.matrix.matrix[j] = a[iter..i].to_vec();
                    }
                }
                imgs.push(img);
                let iter = iter + 28;
            }
        }
    }
    imgs
}

pub fn read_csv(path_of_the_file: &str, lines: &usize) -> Vec<img::Img> {
    const MAXCHAR: usize = 10000; //this numbr cuz each linne has basically 10000 bytes
    let mut string: [u8; MAXCHAR] = [0; MAXCHAR];
    {
        let mut file = File::open(path_of_the_file).unwrap();
        File::read_exact(&mut file, &mut string).unwrap();
    }
    format_data::<MAXCHAR>(&string, lines)
}


