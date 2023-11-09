use strum::IntoEnumIterator;

use crate::{networks::Networks, run, traits::generic_test_case::GenericTestCase};

pub fn runner(network: &Option<Networks>) {
    let test_cases = TestCaseAnd::get_all_generic();
    let layers = 1..4;
    let nodes = 2..6;
    match network {
        Some(Networks::Network1) => run::run(
            "And",
            Networks::Network1,
            &test_cases,
            layers,
            nodes,
            None,
            None,
        ),
        Some(Networks::Network2) => run::run(
            "And",
            Networks::Network2,
            &test_cases,
            layers,
            nodes,
            None,
            None,
        ),
        Some(Networks::Network3) => run::run(
            "And",
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

    use crate::{
        networks::{activation_functions::ActivationFunction, network1::network},
        traits::network_traits::BaseNetwork,
    };

    use super::TestCaseAnd;

    fn default_network() -> network::Network1 {
        network::Network1::new(
            2,
            1,
            3,
            2,
            ActivationFunction::Relu,
            ActivationFunction::Relu,
        )
    }

    #[test]
    fn learn() {
        let test_cases = TestCaseAnd::get_all_generic();
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
        let test_cases = TestCaseAnd::get_all_generic();
        let error = network.test_all(&test_cases, None);
        assert!(error.is_ok());
        assert_eq!(error.unwrap().error, 0.0);
    }
}
pub struct TestCaseAnd {
    input: [f64; 2],
    output: f64,
}
impl TestCaseAnd {
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
    pub fn display(&self) -> String {
        let mut s = String::from("input: ");
        self.input.iter().for_each(|x| {
            s.push_str(&format!("[{:.0}] ", x));
        });
        s.push_str(&format!("\noutput: [{:.0}]", self.output));
        s
    }
    pub fn get_all_generic() -> Vec<GenericTestCase<Vec<f64>, f64>> {
        TestCaseAnd::get_all()
            .iter()
            .map(|x| x.to_generic())
            .collect()
    }
    pub fn get_all() -> [TestCaseAnd; 4] {
        [
            TestCaseAnd {
                input: [0.0, 0.0],
                output: 1.0,
            },
            TestCaseAnd {
                input: [0.0, 1.0],
                output: 0.0,
            },
            TestCaseAnd {
                input: [1.0, 0.0],
                output: 0.0,
            },
            TestCaseAnd {
                input: [1.0, 1.0],
                output: 1.0,
            },
        ]
    }
}
