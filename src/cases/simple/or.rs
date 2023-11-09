use strum::IntoEnumIterator;

use crate::{networks::Networks, run, traits::generic_test_case::GenericTestCase};

pub fn runner(network: &Option<Networks>) {
    let test_cases = TestCaseOr::get_all_generic();
    let layers = 2..3;
    let nodes = 2..6;
    match network {
        Some(Networks::Network1) => run::run(
            "Or",
            Networks::Network1,
            &test_cases,
            layers,
            nodes,
            None,
            None,
        ),
        Some(Networks::Network2) => run::run(
            "Or",
            Networks::Network2,
            &test_cases,
            layers,
            nodes,
            None,
            None,
        ),
        Some(Networks::Network3) => run::run(
            "Or",
            Networks::Network3,
            &test_cases,
            layers,
            nodes,
            None,
            None,
        ),
        None => {
            Networks::iter().for_each(|network| {
                runner(&Some(network));
            });
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::cases::simple::or::TestCaseOr;
    use crate::networks::activation_functions::ActivationFunction;
    use crate::networks::network1::{layer::Layer, network, node::Node};
    use crate::traits::network_traits::BaseNetwork;

    #[test]
    fn known_good() {
        let test_cases = TestCaseOr::get_all_generic();
        let mut network = network::Network1 {
            layers: vec![
                Layer {
                    nodes: vec![
                        Node::new_paths(vec![1.0, 0.0]),
                        Node::new_paths(vec![0.0, 1.0]),
                        Node::new_paths(vec![0.0, 0.0]),
                    ],
                },
                Layer {
                    nodes: vec![Node::new_paths(vec![1.0, 1.0, 1.0])],
                },
            ],
            activation_fn: ActivationFunction::Relu,
            output_activation_fn: ActivationFunction::Relu,
        };
        let error = network.test_all(&test_cases, None).unwrap();
        assert_eq!(error.error, 0.0);
    }

    fn default_network() -> network::Network1 {
        network::Network1::new(
            2,
            1,
            3,
            1,
            ActivationFunction::Relu,
            ActivationFunction::Relu,
        )
    }

    #[test]
    fn learn() {
        let test_cases = TestCaseOr::get_all_generic();
        for _ in 0..20 {
            let mut network = default_network();
            match network.learn(&test_cases, Some(100_000), None, None, None) {
                Ok(_) => (),
                Err(e) => panic!("{}", e),
            }
            test(network);
        }
    }

    fn test(mut network: network::Network1) {
        let error = network.test_all(&TestCaseOr::get_all_generic(), None);
        assert!(error.is_ok());
        assert_eq!(error.unwrap().error, 0.0);
    }
}
pub struct TestCaseOr {
    input: [f64; 2],
    output: f64,
}
impl TestCaseOr {
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
            .map(|(x, y)| (x - y).powi(2))
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
        TestCaseOr::get_all()
            .iter()
            .map(|x| x.to_generic())
            .collect()
    }
    pub fn get_all() -> [TestCaseOr; 4] {
        [
            TestCaseOr {
                input: [0.0, 0.0],
                output: 0.0,
            },
            TestCaseOr {
                input: [0.0, 1.0],
                output: 1.0,
            },
            TestCaseOr {
                input: [1.0, 0.0],
                output: 1.0,
            },
            TestCaseOr {
                input: [1.0, 1.0],
                output: 1.0,
            },
        ]
    }
}
