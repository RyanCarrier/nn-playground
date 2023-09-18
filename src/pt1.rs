use crate::GenericTestCase;

#[cfg(test)]
mod tests {
    use crate::layer::Layer;
    use crate::node::Node;
    use crate::TestCaseOr;
    use crate::{network, GenericTestCase};

    fn test_cases() -> Vec<GenericTestCase> {
        TestCaseOr::get_all()
            .iter()
            .map(|x| x.to_generic())
            .collect()
    }
    #[test]
    fn known_good() {
        let test_cases = test_cases();
        let mut network = network::Network {
            layers: vec![
                Layer {
                    nodes: vec![
                        Node {
                            paths: vec![1.0, 0.0],
                            old_paths: vec![0.0, 0.0],
                            value: 0.0,
                        },
                        Node {
                            paths: vec![0.0, 1.0],
                            old_paths: vec![0.0, 0.0],
                            value: 0.0,
                        },
                        Node {
                            paths: vec![0.0, 0.0],
                            old_paths: vec![0.0, 0.0],
                            value: 0.0,
                        },
                    ],
                },
                Layer {
                    nodes: vec![Node {
                        paths: vec![1.0, 1.0, 1.0],
                        old_paths: vec![],
                        value: 0.0,
                    }],
                },
            ],
            output_fn: |x| if x > 0.5 { 1.0 } else { 0.0 },
        };
        let error = network.test_all(&test_cases).unwrap();
        assert_eq!(error, 0.0);
    }
    #[test]
    fn learn_10000() {
        test_iter(10000);
    }
    #[test]
    fn learn_1000() {
        test_iter(1000);
    }
    #[test]
    fn learn_10() {
        test_iter(10);
    }
    #[test]
    fn learn_5() {
        test_iter(5);
    }
    #[test]
    fn learn_1() {
        test_iter(1);
    }
    #[test]
    fn learn_0() {
        //lol
        //this is only here cause it was working with base of 0.5 paths and i was like???
        test_iter(0);
    }
    fn default_network() -> network::Network {
        network::Network::new(2, 1, 3, 1, Some(|x| if x > 0.5 { 1.0 } else { 0.0 }))
    }

    fn test_iter(i: usize) {
        let test_cases = test_cases();
        let mut network = default_network();
        match network.learn(&test_cases, Some(i), None) {
            Ok(_) => (),
            Err(e) => panic!("{}", e),
        }
        test(network);
    }

    #[test]
    fn auto_learn() {
        let test_cases = test_cases();
        let mut network = default_network();
        match network.auto_learn(&test_cases) {
            Ok(_) => (),
            Err(e) => panic!("{}", e),
        }
        test(network);
    }

    fn test(mut network: network::Network) {
        let error = network.test_all(&test_cases());
        assert!(error.is_ok());
        assert_eq!(error.unwrap(), 0.0);
    }
}
pub struct TestCaseOr {
    input: [f64; 2],
    output: f64,
}
impl TestCaseOr {
    pub fn to_generic(&self) -> GenericTestCase {
        GenericTestCase {
            input: self.input.to_vec(),
            output: vec![self.output],
            display: self.display(),
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
