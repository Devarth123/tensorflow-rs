//will be optimizing the code very soon.
use crate::ml::f32::activation;
use crate::ml::f32::matrix::MatrixStruct;
use crate::ml::f32::img::Img;


// use crate::ml::f32::img::Img;

pub struct NeuralNetwork {
    parameters: Box<[usize]>, //the first index will be the input layer (parameters[0]) , the last index will be the output layer (parameters[parameters.len()-1]) and lastly the rest of the vaues will be the hidden_layer (parameters[1..parameters.len()-2])
    learning_rate: f32,
    hidden_weights: MatrixStruct,
    output_weights: MatrixStruct,
}

impl NeuralNetwork {
    pub fn from(parameters_: &Box<[usize]>, learning_rate_: &f32) -> NeuralNetwork {
        NeuralNetwork {
            parameters: Box::clone(parameters_),
            learning_rate: *learning_rate_,
            hidden_weights: MatrixStruct::from(&parameters_[1], &parameters_[0]),
            output_weights: MatrixStruct::from(
                &parameters_[parameters_.len() - 1],
                &parameters_[1],
            ),
        }
    }
    pub fn train(
        nn: &mut NeuralNetwork,
        input: &MatrixStruct,
        output: &MatrixStruct,
    ) {
        {
            //doing teh basic things to find the errors
            let hidden_inputs = MatrixStruct::dot(&nn.hidden_weights, input);
            let hidden_ouputs = activation::apply(&(0), &hidden_inputs);
            let final_inputs = MatrixStruct::dot(&nn.output_weights, &hidden_ouputs);
            let final_outputs = activation::apply(&(0), &final_inputs);
            //finding the error
            let output_errors = MatrixStruct::subtract(output, &final_outputs); // and we are done
            let hidden_errors = MatrixStruct::dot(
                &(MatrixStruct::transpose(&nn.output_weights)),
                &output_errors,
            );
            // applying the sigmoid_prime method on the final outputa and mutiplying it with the output
            // error
            // let sigmoid_prime_matrix = activation::sigmoid_prime64(&final_outputs);
            let multiplied_matrix = MatrixStruct::multiply(
                &output_errors,
                &(activation::sigmoid_prime(&final_outputs)),
            );
            let dot_matrix =
                MatrixStruct::dot(&multiplied_matrix, &MatrixStruct::transpose(&hidden_ouputs));
            let scaled_matrix = MatrixStruct::scale(&nn.learning_rate, &dot_matrix);
            nn.output_weights = MatrixStruct::add(&nn.output_weights, &scaled_matrix);
            let multiplied_matrix = MatrixStruct::multiply(
                &hidden_errors,
                &(activation::sigmoid_prime(&hidden_ouputs)),
            );
            let dot_matrix = MatrixStruct::dot(&multiplied_matrix, &MatrixStruct::transpose(input));
            let scaled_matrix = MatrixStruct::scale(&nn.learning_rate, &dot_matrix);
            nn.hidden_weights = MatrixStruct::add(&nn.hidden_weights, &scaled_matrix);
        }
    }

    pub fn train_batch_imgs(
        nn: &mut NeuralNetwork,
        imgs: &mut Vec<Img>,
        ouput_row: &usize,
    ) {
        for i in imgs.iter() {
            let flatten = MatrixStruct::flatten(&i.matrix);
            //output = matrix::from(10, 1) // in this case
            NeuralNetwork::train(nn, &flatten, &(MatrixStruct::from(ouput_row, &1)));
            
        }
    }
}
