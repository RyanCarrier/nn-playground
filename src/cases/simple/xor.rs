use strum::IntoEnumIterator;

use crate::{networks::Networks, run, traits::generic_test_case::GenericTestCase};

pub fn runner(network: &Option<Networks>) {
    let test_cases = TestCaseXor::get_all_generic();
    let layers = 3..5;
    let nodes = 4..5;
    match network {
        Some(Networks::Network1) => run::run("Xor", Networks::Network1, &test_cases, layers, nodes),
        Some(Networks::Network2) => run::run("Xor", Networks::Network2, &test_cases, layers, nodes),
        Some(Networks::Network3) => run::run("Xor", Networks::Network3, &test_cases, layers, nodes),
        None => {
            Networks::iter().for_each(|network| {
                runner(&Some(network));
            });
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::networks::network3::network;
    use crate::traits::network_traits::BaseNetwork;

    fn default_network() -> network::Network3 {
        network::Network3::new(2, 1, 3, 2, |x| x.max(0.0))
    }

    #[test]
    fn learn() {
        let test_cases = TestCaseXor::get_all_generic();
        let mut network = default_network();
        match network.learn(&test_cases, Some(100_000), None, None, None, |_| 1.0) {
            Ok(_) => (),
            Err(e) => panic!("{}", e),
        }
        test(network);
    }

    fn test(mut network: network::Network3) {
        let error = network.test_all(&TestCaseXor::get_all_generic(), None);
        assert!(error.is_ok());
        assert_eq!(error.unwrap().error, 0.0);
    }
}
pub struct TestCaseXor {
    input: [f64; 2],
    output: f64,
}
impl TestCaseXor {
    pub fn to_generic(&self) -> GenericTestCase<Vec<f64>, f64> {
        GenericTestCase {
            input: self.input.to_vec(),
            output: [self.output].to_vec(),
            output_nodes: 1,
            display: self.display(),
            input_transformer: |x| x.to_vec(),
            output_transformer: |x| (*x.first().unwrap()).max(1.0).min(0.0),
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
        TestCaseXor::get_all()
            .iter()
            .map(|x| x.to_generic())
            .collect()
    }
    pub fn get_all() -> [TestCaseXor; 4] {
        [
            TestCaseXor {
                input: [0.0, 0.0],
                output: 0.0,
            },
            TestCaseXor {
                input: [0.0, 1.0],
                output: 1.0,
            },
            TestCaseXor {
                input: [1.0, 0.0],
                output: 1.0,
            },
            TestCaseXor {
                input: [1.0, 1.0],
                output: 0.0,
            },
        ]
    }
}
