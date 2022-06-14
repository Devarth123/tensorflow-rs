//will be optimizing the code very soon.
use crate::ml::f32::activation;
use crate::ml::f32::matrix;

pub struct NeuralNetwork<'a> {
    parameters: &'a [u16], //the first index will be the input layer (parameters[0]) , the last index will be the output layer (parameters[parameters.len()-1]) and lastly the rest of the vaues will be the hidden_layer (parameters[1..parameters.len()-2])
    learning_rate: f32,
    hidden_weights: matrix::MatrixStruct,
    output_weights: matrix::MatrixStruct,
}

impl <'a>NeuralNetwork<'a>{
    pub fn new(
        parameters_: &'a [u16],
        learning_rate_: &'a f32,
    ) -> NeuralNetwork<'a> {
        NeuralNetwork {
            parameters: parameters_,
            learning_rate: *learning_rate_,
            hidden_weights: matrix::matrix_create(&parameters_[1], &parameters_[0]),
            output_weights: matrix::matrix_create(
                &parameters_[parameters_.len() - 1],
                &parameters_[1],
            ),
        }
    }
}
pub fn neural_network_train64(
    nn: &mut NeuralNetwork,
    input: &matrix::MatrixStruct,
    output: &matrix::MatrixStruct,
) {
    {
        //doing teh basic things to find the errors
        let hidden_inputs = matrix::dot(&nn.hidden_weights, input);
        let hidden_ouputs = activation::apply(&(0), &hidden_inputs);
        let final_inputs = matrix::dot(&nn.output_weights, &hidden_ouputs);
        let final_outputs = activation::apply(&(0), &final_inputs);
        //finding the error
        let output_errors = matrix::subtract(output, &final_outputs); // and we are done
        let hidden_errors = matrix::dot(&(matrix::transpose(&nn.output_weights)), &output_errors);
        // applying the sigmoid_prime method on the final outputa and mutiplying it with the output
        // error
        // let sigmoid_prime_matrix = activation::sigmoid_prime64(&final_outputs);
        let multiplied_matrix =
            matrix::multiply(&output_errors, &(activation::sigmoid_prime(&final_outputs)));
        let dot_matrix = matrix::dot(&multiplied_matrix, &matrix::transpose(&hidden_ouputs));
        let scaled_matrix = matrix::scale(&nn.learning_rate, &dot_matrix);
        nn.output_weights = matrix::add(&nn.output_weights, &scaled_matrix);
        let multiplied_matrix =
            matrix::multiply(&hidden_errors, &(activation::sigmoid_prime(&hidden_ouputs)));
        let dot_matrix = matrix::dot(&multiplied_matrix, &matrix::transpose(input));
        let scaled_matrix = matrix::scale(&nn.learning_rate, &dot_matrix);
        nn.hidden_weights = matrix::add(&nn.hidden_weights, &scaled_matrix);
    }
}
