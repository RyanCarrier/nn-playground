#[derive(Clone, Copy)]
pub enum ActivationFunction {
    Sigmoid,
    Relu,
    Linear,
}

impl ActivationFunction {
    pub fn forward(&self, x: f64) -> f64 {
        match self {
            ActivationFunction::Sigmoid => 1.0 / (1.0 + (-x).exp()),
            ActivationFunction::Relu => {
                if x > 0.0 {
                    x
                } else {
                    0.0
                }
            }
            ActivationFunction::Linear => x,
        }
    }

    pub fn derivative(&self, x: f64) -> f64 {
        match self {
            ActivationFunction::Sigmoid => {
                let y = self.forward(x);
                y * (1.0 - y)
            }
            ActivationFunction::Relu => {
                if x > 0.0 {
                    1.0
                } else {
                    0.0
                }
            }
            ActivationFunction::Linear => 1.0,
        }
    }
}
