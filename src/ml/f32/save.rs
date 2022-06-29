use crate::ml::f32::nn::NeuralNetwork;
use std::fs::File;
use std::io::prelude::*;
use crate::ml::f32::matrix::MatrixStruct;
use std::process::Command;

#[allow(dead_code)]
struct Save {
    path: String,
    nn: NeuralNetwork,
}

impl Save {
    #[allow(dead_code)]
    pub fn from(nn_: &NeuralNetwork, path_: &String) -> Save {
        Save {
            path: {
                if path_ != "" {
                    path_.to_string()
                } else {
                    panic!("where is the path big brother?, didnt ask for a null string :< ");
                }
            },
            nn: NeuralNetwork::clone(&nn_),
        }
    }
    #[allow(dead_code)]
    pub fn mkdirs(&self) {
        let cmd = Command::new("sh")
            .arg(format!(
                "-c mkdir -v -p {} {} {}",
                (self.path.clone() + &"/NeuralNetwork/output"),
                (self.path.clone() + &"/NeuralNetwork/hidden"),
                (self.path.clone() + &"/NeuralNetwork/input")
            ))
            .output()
            .unwrap_or_else(|err| panic!("wahhahaha wtf happened ahhhhh, error message {}", err));
        println!("{:?}", cmd.stdout);
    }

    #[allow(dead_code)]
    pub fn touch_files(&self){
        let mut cmd = Command::new("touch");
        let len = self.nn.parameters.len();
        for i in 0..len{
            match i{
                0 => {
                    cmd.arg(self.path.clone()+&format!("NeuralNetwork/input/{}NN.csv", i));
                }
                l if l == len => {
                    cmd.arg(self.path.clone()+&format!("NeuralNetwork/output/{}NN.csv", i));
                }
                _ => {
                    cmd.arg(self.path.clone()+&format!("NeuralNetwork/hidden/{}NN.csv", i));
                }
            }
        }
    }
    #[allow(dead_code)]
    pub fn write_matrix(m: &MatrixStruct, path: &String) -> std::io::Result<()>{
        let mut file = File::create(path)?;
        for i in 0..m.rows{
            for j in 0..m.columns{
              file.write_all(&format!("{}\n", m.matrix[i][j]).as_bytes())?; 
            }
        } 
        Ok(())
    }
    #[allow(dead_code)]
    pub fn write_all(&self) -> std::io::Result<()> {
        let mut file = File::create(self.path.clone()+&"NeuralNetwork/hidden/1NN.csv")?;
        for i in 0..self.nn.hidden_weights.rows{
            for j in 0..self.nn.hidden_weights.columns{
                file.write_all(&format!("{}\n", self.nn.hidden_weights.matrix[i][j]).as_bytes())?;
            }
        }
        let mut file  = File::create(self.path.clone()+&"NeuralNetwork/output/1NN.csv")?;
        for i in 0..self.nn.output_weights.rows{
            for j in 0..self.nn.output_weights.columns {
                file.write_all(&format!("{}\n", self.nn.hidden_weights.matrix[i][j]).as_bytes())?;
            }
        }
        Ok(())
    }
}

