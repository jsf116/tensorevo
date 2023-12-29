// TODO: Documentation
//       https://github.com/mfajnberg/tensorevo/issues/22

use serde::{Deserialize, Serialize};

use crate::activation::Activation;
use crate::tensor::Tensor;


#[derive(Clone, Deserialize, Serialize, PartialEq, Debug)]
#[serde(bound = "")]
pub struct Layer<T: Tensor> {
    pub weights: T,
    pub biases: T,
    pub activation: Activation<T>
}

impl<T: Tensor> Layer<T> {
    pub fn new(weights: T, biases: T, activation: Activation<T>) -> Self {
        Self { weights, biases, activation }
    }

    pub fn feed_forward(&self, input: &T) -> (T, T) {
        let weighted_input = &self.weights.dot(input) + &self.biases;
        let activation = self.activation.call(&weighted_input);
        (weighted_input, activation)
    }
}

#[cfg(test)]
mod tests {
    use ndarray::array;

    use super::*;

    #[test]
    fn test() {
    }    
}
