use std::ops::Range;

use strum::IntoEnumIterator;

use crate::{
    networks::{activation_functions::ActivationFunction, Networks},
    run,
    traits::generic_test_case::GenericTestCase,
};

const TITLE: &str = "math_linear";
const LAYERS: Range<usize> = 3..5;
const NODES: Range<usize> = 5..7;

pub fn runner(network: &Option<Networks>) {
    let test_cases = MathLinear::get_all_generic();
    match network {
        Some(Networks::Network1) => run::run(
            TITLE,
            Networks::Network1,
            &test_cases,
            LAYERS,
            NODES,
            Some(ActivationFunction::Relu),
            Some(ActivationFunction::Linear),
        ),
        Some(Networks::Network2) => run::run(
            TITLE,
            Networks::Network2,
            &test_cases,
            LAYERS,
            NODES,
            Some(ActivationFunction::Relu),
            Some(ActivationFunction::Linear),
        ),
        Some(Networks::Network3) => run::run(
            TITLE,
            Networks::Network3,
            &test_cases,
            LAYERS,
            NODES,
            Some(ActivationFunction::Relu),
            Some(ActivationFunction::Linear),
        ),
        None => {
            Networks::iter().for_each(|network| {
                runner(&Some(network));
            });
        }
    }
}
//MathLinear will take the 3 inputs, (a,b,c) and return a*b + c
//So no activation function on final layer is needed
pub struct MathLinear {
    input: [f64; 3],
    output: f64,
}
impl MathLinear {
    pub fn to_generic(&self) -> GenericTestCase<Vec<f64>, f64> {
        GenericTestCase {
            input: self.input.to_vec(),
            output: [self.output].to_vec(),
            output_nodes: 1,
            display: self.display(),
            input_transformer: |x| x.to_vec(),
            output_transformer: |x| *x.first().unwrap(),
        }
    }
    pub fn error_fn(output: &Vec<f64>, expected_output: &Vec<f64>) -> Vec<f64> {
        output
            .iter()
            .zip(expected_output.iter())
            .map(|(x, y)| (x - y).powi(2) / 2.0)
            .collect()
    }
    pub fn display(&self) -> String {
        let mut s = String::from("input: ");
        self.input.iter().for_each(|x| {
            s.push_str(&format!("[{:.0}] ", x));
        });
        s.push_str(&format!("\noutput: [{:.0}]", self.output));
        s
    }
    pub fn get_all_generic() -> Vec<GenericTestCase<Vec<f64>, f64>> {
        MathLinear::get_all()
            .iter()
            .map(|x| x.to_generic())
            .collect()
    }
    pub fn get_all() -> Vec<MathLinear> {
        let mut test_cases = Vec::new();
        let range = 0..4;
        for a in range.clone() {
            for b in range.clone() {
                for c in range.clone() {
                    test_cases.push(MathLinear {
                        input: [a as f64, b as f64, c as f64],
                        output: (a as f64) * (b as f64) + (c as f64),
                    });
                }
            }
        }
        test_cases
    }
}
