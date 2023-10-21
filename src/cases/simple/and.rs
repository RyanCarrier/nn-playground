use crate::{networks::Networks, run, traits::generic_test_case::GenericTestCase};

pub fn runner() {
    let test_cases = TestCaseAnd::get_all_generic();
    run::run("And", Networks::Network1, &test_cases, 1..4, 2..6);
    run::run("And", Networks::Network2, &test_cases, 1..4, 2..6);
}
#[cfg(test)]
mod tests {

    use crate::{networks::network1::network, traits::network_traits::BaseNetwork};

    use super::TestCaseAnd;

    fn default_network() -> network::Network1 {
        network::Network1::new(2, 1, 3, 2, None)
    }

    #[test]
    fn learn() {
        let test_cases = TestCaseAnd::get_all_generic();
        for _ in 0..20 {
            let mut network = default_network();
            match network.learn(&test_cases, Some(100_000), None, None) {
                Ok(_) => (),
                Err(e) => panic!("{}", e),
            }
            test(network);
        }
    }

    fn test(mut network: network::Network1) {
        let test_cases = TestCaseAnd::get_all_generic();
        let error = network.test_all(&test_cases, Some(TestCaseAnd::error_fn));
        assert!(error.is_ok());
        assert_eq!(error.unwrap().error, 0.0);
    }
}
pub struct TestCaseAnd {
    input: [f64; 2],
    output: f64,
}
impl TestCaseAnd {
    pub fn error_fn(output: &Vec<f64>, expected_output: &Vec<f64>) -> Vec<f64> {
        output
            .iter()
            .zip(expected_output.iter())
            .map(|(x, y)| (x - y).powi(2))
            .collect()
    }
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
