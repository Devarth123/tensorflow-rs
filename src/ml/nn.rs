use crate::ml::matrix;

pub struct NeuralNetwork64<'a>{
    parameters: &'a [u16], //the first index will be the input layer (parameters[0]) , the last index will be the output layer (parameters[parameters.len()-1]) and lastly the rest of the vaues will be the hidden_layer (parameters[1..parameters.len()-2])
    learning_rate: f32,
    hidden_layer: matrix::MatrixStruct64<'a>, 
    output_layer: matrix::MatrixStruct64<'a>
}

pub fn neural_network64<'a>(parameters_: &'a [u16], learning_rate_: &'a f32) -> NeuralNetwork64<'a>{
  let hidden = matrix::matrix_create64(&parameters_[0], &parameters_[1]);
  let output = matrix::matrix_create64(&parameters_[1], &parameters_[2]);
  NeuralNetwork64{
      parameters:  parameters_,
      learning_rate: *learning_rate_,
      hidden_layer: hidden,
      output_layer: output
  } 
}

pub fn neural_network_train64(nn: &mut NeuralNetwork64, input: &matrix::MatrixStruct64, output: &matrix::MatrixStruct64){
     
}
