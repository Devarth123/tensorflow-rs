        pub fn str_to_f32(str_vec: Vec<String>) -> Vec<f32>{
            let mut from_str_f32: Vec<f32> = Vec::with_capacity(str_vec.len()); 
            for i in str_vec{
                for j in i.chars(){
                   if j !=','{
                       let push: f32 = j .to_string()                                  // .trim()
                                        .parse()
                                        .expect("the method str_to_f32 is not working at line 41");
                       from_str_f32.push(push); 
                   } 
                } 
           }
        from_str_f32
    }
