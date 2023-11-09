use std::ops::Range;

use strum::IntoEnumIterator;

use crate::{
    networks::{activation_functions::ActivationFunction, Networks},
    run,
    traits::generic_test_case::GenericTestCase,
};

const TITLE: &str = "math_quadratic";
const LAYERS: Range<usize> = 5..6;
const NODES: Range<usize> = 8..11;

pub fn runner(network: &Option<Networks>) {
    let test_cases = MathQuadratic::get_all_generic();
    match network {
        Some(Networks::Network1) => run::run(
            TITLE,
            Networks::Network1,
            &test_cases,
            LAYERS,
            NODES,
            None,
            None,
        ),
        Some(Networks::Network2) => run::run(
            TITLE,
            Networks::Network2,
            &test_cases,
            LAYERS,
            NODES,
            None,
            None,
        ),
        Some(Networks::Network3) => run::run(
            TITLE,
            Networks::Network3,
            &test_cases,
            LAYERS,
            NODES,
            None,
            Some(ActivationFunction::Linear),
        ),
        None => {
            Networks::iter().for_each(|network| {
                runner(&Some(network));
            });
        }
    }
}
//MathQuadratic will take the 3 inputs, (a,b,c) and return a^2 + 2*b + c
//So no activation function on final layer is needed
pub struct MathQuadratic {
    input: [f64; 3],
    output: f64,
}
impl MathQuadratic {
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
        MathQuadratic::get_all()
            .iter()
            .map(|x| x.to_generic())
            .collect()
    }
    pub fn get_all() -> Vec<MathQuadratic> {
        let mut test_cases = Vec::new();
        for a in 0..4 {
            for b in 0..4 {
                for c in 0..4 {
                    test_cases.push(MathQuadratic {
                        input: [a as f64, b as f64, c as f64],
                        output: (a as f64).powi(2) + 2.0 * (b as f64) + (c as f64),
                    });
                }
            }
        }
        test_cases
    }
}
