use crate::ml::matrix;

pub struct NeuralNetwork64<'a>{
    parameters: &'a [u16], //the first index will be the input layer (parameters[0]) , the last index will be the output layer (parameters[parameters.len()-1]) and lastly the rest of the vaues will be the hidden_layer (parameters[1..parameters.len()-2])
    learning_rate: f32,
    hidden_weights: matrix::MatrixStruct64<'a>, 
    output_weights: matrix::MatrixStruct64<'a>
}

pub fn neural_network64<'a>(parameters_: &'a [u16], learning_rate_: &'a f32) -> NeuralNetwork64<'a>{
  NeuralNetwork64{
      parameters:  parameters_,
      learning_rate: *learning_rate_,
      hidden_weights: matrix::matrix_create64(&parameters_[1], &parameters_[0]),
      output_weights: matrix::matrix_create64(&parameters_[parameters_.len()-1], &parameters_[1])
  } 
}
pub fn neural_network_train64(nn: &mut NeuralNetwork64, input: &matrix::MatrixStruct64, output: &matrix::MatrixStruct64){
       // let hidden_inputs = matrix::dot64(nn.hidden_weights, input) 
        
}
