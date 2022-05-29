use std::fs;

pub fn breaker(not_formated_string: &str) -> Vec<String> {
    // print!("{not_formated_string}");
    let mut formated_string: Vec<String> = Vec::new();
    let mut push_str = String::new(); 
    for i in not_formated_string.chars(){
        if i == '\n'{
           formated_string.push(push_str.clone()); 
        }else{
            push_str.push(i); 
        }
    }
    formated_string
}

pub fn count_new_lines(string: &str) -> u16{
    let mut new_lines = 0;
    for i in string.chars(){
        if i == '\n'{
            new_lines+=1;
        }
    }
    new_lines
}
pub fn read_csv(path_of_the_file: &str) -> Vec<Vec<f32>>{
   let data: String = fs::read_to_string(&path_of_the_file).unwrap();
   let data = breaker(&data); 
   let mut str_to_f32: Vec<Vec<f32>> = Vec::new();
   for i in data{
    let mut push_vec: Vec<f32> = vec![];
    for j in i.chars() {
        if j != ','{
            let push_element: f32 = j
                                    .to_string()
                                    .parse()
                                    .expect("The methmod is not working at line 36\n {}");
            push_vec.push(push_element);
            } 
    }
    str_to_f32.push(push_vec);
   }
    str_to_f32
}
