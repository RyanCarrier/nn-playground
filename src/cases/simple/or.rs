use crate::{generic_test_case::GenericTestCase, run};

pub fn runner() {
    let test_cases = TestCaseOr::get_all_generic();
    run::run("Or", &test_cases, 1..3, 2..6);
}
#[cfg(test)]
mod tests {
    use crate::cases::simple::or::TestCaseOr;
    use crate::network1::{layer::Layer, network, node::Node};
    use crate::network_traits::BaseNetwork;

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
            output_fn: |x| if x > 0.5 { 1.0 } else { 0.0 },
        };
        let error = network.test_all(&test_cases).unwrap();
        assert_eq!(error, 0.0);
    }

    fn default_network() -> network::Network1 {
        network::Network1::new(2, 1, 3, 1, Some(|x| x.min(1.0).max(0.0)))
    }

    #[test]
    fn learn() {
        let test_cases = TestCaseOr::get_all_generic();
        for _ in 0..20 {
            let mut network = default_network();
            match network.learn(&test_cases, Some(100_000), None) {
                Ok(_) => (),
                Err(e) => panic!("{}", e),
            }
            test(network);
        }
    }

    fn test(mut network: network::Network1) {
        let error = network.test_all(&TestCaseOr::get_all_generic());
        assert!(error.is_ok());
        assert_eq!(error.unwrap(), 0.0);
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
            output: self.output,
            output_nodes: 1,
            display: self.display(),
            input_transformer: |x| x.to_vec(),
            output_transformer: |x| *x.first().unwrap(),
            output_error: |x, y| (x - y).abs(),
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