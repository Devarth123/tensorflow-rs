//will be optimizing the code very soon.
use crate::ml::matrix;
use crate::ml::activation;

pub struct NeuralNetwork64<'a>{
    parameters: &'a [u16], //the first index will be the input layer (parameters[0]) , the last index will be the output layer (parameters[parameters.len()-1]) and lastly the rest of the vaues will be the hidden_layer (parameters[1..parameters.len()-2])
    learning_rate: f64,
    hidden_weights: matrix::MatrixStruct64, 
    output_weights: matrix::MatrixStruct64
}

pub fn neural_network64<'a>(parameters_: &'a [u16], learning_rate_: &'a f64) -> NeuralNetwork64<'a>{
  NeuralNetwork64{
      parameters:  parameters_,
      learning_rate: *learning_rate_,
      hidden_weights: matrix::matrix_create64(&parameters_[1], &parameters_[0]),
      output_weights: matrix::matrix_create64(&parameters_[parameters_.len()-1], &parameters_[1])
  } 
}
pub fn neural_network_train64(nn: &mut NeuralNetwork64, input: &matrix::MatrixStruct64, output: &matrix::MatrixStruct64){
    {
        //doing teh basic things to find the errors
        let hidden_inputs = matrix::dot64(&nn.hidden_weights, input);
        let hidden_ouputs = activation::apply(&(0), &hidden_inputs); 
        let final_inputs = matrix::dot64(&nn.output_weights, &hidden_ouputs);
        let final_outputs = activation::apply(&(0), &final_inputs);
        //finding the error
        let output_errors =  matrix::subtract64(&output, &final_outputs);   // and we are done
        let hidden_errors  = matrix::dot64(&(matrix::transpose64(&nn.output_weights)), &output_errors);
        // applying the sigmoid_prime method on the final outputa and mutiplying it with the output
        // error  
        // let sigmoid_prime_matrix = activation::sigmoid_prime64(&final_outputs); 
        let multiplied_matrix = matrix::multiply64(&output_errors , &(activation::sigmoid_prime64(&final_outputs))); 
        let dot_matrix = matrix::dot64(&multiplied_matrix, &matrix::transpose64(&hidden_ouputs));
        let scaled_matrix = matrix::scale(&nn.learning_rate, &dot_matrix);
        nn.output_weights = matrix::add64(&nn.output_weights, &scaled_matrix);  
        let multiplied_matrix = matrix::multiply64(&hidden_errors, &(activation::sigmoid_prime64(&hidden_ouputs)));
        let dot_matrix = matrix::dot64(&multiplied_matrix, &matrix::transpose64(&input));
        let scaled_matrix = matrix::scale(&nn.learning_rate, &dot_matrix);
        nn.hidden_weights = matrix::add64(&nn.hidden_weights, &scaled_matrix);
    }
}
