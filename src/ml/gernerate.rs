extern crate rand;

use std::collections::HashMap;
use rand::{Rng, thread_rng};


pub fn storing_labels<'a>(formated_data: &'a Vec<f32>, labels: &mut HashMap<u16, &'a[f32]>){
         labels.insert(formated_data[0] as u16, &formated_data[1..]); 
 }
pub fn gernerate_imgs<'a>(labels: HashMap<u16, &'a Vec<Vec<f32>>>, range: &u32) -> &'a Vec<Vec<f32>>{
       let random_img: u16 = rand::thread_rng().gen_range(0, *range) as u16;
       // let random_img: u16 = crate::Rng.get_range(0, range);
       match labels.get(&random_img){
              Some(vec) => *vec,
              None => panic!("the random gernerate_imgs() method is not working at line 49") 
         }
    }

