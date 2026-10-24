use rand::{self, Rng};

struct Layer {
    weights: Vec<Vec<f32>>,
    biases: Vec<f32>,
}

impl Layer {
    fn new(input_count: usize, output_count: usize) -> Layer {
        let mut rng = rand::thread_rng();
        let mut weights = vec![vec![0.0; input_count]; output_count];
        let mut biases = vec![0.0; output_count];

        for i in 0..output_count {
            for j in 0..input_count {
                weights[i][j] = rng.gen_range(-1.0..1.0);
            }
            biases[i] = rng.gen_range(-1.0..1.0);
        }

        Layer {
            weights: weights,
            biases: biases,
        }
    }

    fn forward(&self, inputs: &[f32]) -> Vec<f32> {
        let mut outputs = vec![0.0; self.biases.len()];

        for i in 0..self.biases.len() {
            let mut sum = self.biases[i];
            for j in 0..inputs.len() {
                sum += inputs[j] * self.weights[i][j];
            }
            outputs[i] = sum.max(0.0);
        }
        outputs
    }
}

pub struct Network {
    layers: Vec<Layer>,
}

impl Network {
    pub fn new(topology: &[usize]) -> Network {
        let mut layers = Vec::new();
        for i in 0..topology.len() - 1 {
            layers.push(Layer::new(topology[i], topology[i + 1]));
        }
        Network { layers }
    }

    pub fn decide(&self, inputs: &[f32]) -> usize {
        let mut current_signals = inputs.to_vec();

        for layer in &self.layers {
            current_signals = layer.forward(&current_signals);
        }

        if current_signals.is_empty() {
            return 0;
        }

        let mut max_index = 0;
        let mut max_value = current_signals[0];

        for i in 1..current_signals.len() {
            if current_signals[i] > max_value {
                max_value = current_signals[i];
                max_index = i;
            }
        }

        max_index
    }
}
