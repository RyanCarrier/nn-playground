pub mod layer;
mod network;
pub mod node;
pub mod path;

fn main() {
    main_1();
    // main_2();
}

fn main_1() {
    let test_cases = TestCaseOr::get_all()
        .iter()
        .map(|x| x.to_generic())
        .collect();
    for i in [10, 100, 1000, 10000].iter() {
        run_1(*i, &test_cases);
    }
}
fn run_1(iter: usize, test_cases: &Vec<GenericTestCase>) {
    let output_fn: fn(f64) -> f64 = |x| {
        if x > 0.5 {
            1.0
        } else {
            0.0
        }
    };
    let mut network = network::Network::new(2, 1, 3, 1, Some(output_fn));
    match network.auto_learn(&test_cases, iter) {
        Ok(_) => (),
        Err(e) => panic!("{}", e),
    }
    match network.test_all(&test_cases) {
        Ok(result) => {
            println!("{}: {}", iter, result);
        }
        Err(e) => panic!("{}", e),
    }
}
fn main_2() {
    let test_cases = TestCaseOrAnd::get_all()
        .iter()
        .map(|x| x.to_generic())
        .collect();
    for i in [10, 1000, 100000].iter() {
        let mut network = network::Network::new(2, 1, 2, 1, None);
        match network.auto_learn(&test_cases, *i) {
            Ok(_) => (),
            Err(e) => panic!("{}", e),
        }
        match network.test_all(&test_cases) {
            Ok(result) => {
                println!("{}: {}", i, result);
            }
            Err(e) => panic!("{}", e),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::layer::Layer;
    use crate::network;
    use crate::node::Node;
    use crate::TestCaseOr;

    #[test]
    fn pt1() {
        let test_cases = TestCaseOr::get_all()
            .iter()
            .map(|x| x.to_generic())
            .collect();
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
}
pub struct GenericTestCase {
    input: Vec<f64>,
    output: Vec<f64>,
    display: String,
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
#[derive(Clone, Copy)]
pub struct TestCaseOrAnd {
    pub input: [f64; 3],
    pub output: f64,
}
impl TestCaseOrAnd {
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
    pub fn get_all() -> [TestCaseOrAnd; 8] {
        let mut result: [TestCaseOrAnd; 8] = [TestCaseOrAnd {
            input: [0.0, 0.0, 0.0],
            output: 0.0,
        }; 8];
        //k=0 or, k=1 and
        //haha yeah nice
        for i in 0..1 {
            for j in 0..1 {
                for k in 0..1 {
                    result[i + 2 * j + 4 * k] = TestCaseOrAnd {
                        input: [i as f64, j as f64, k as f64],
                        //shoudl probably verify this is correct too...
                        output: ((!k & (i | j)) | (k & (i & j))) as f64,
                    };
                }
            }
        }
        result
    }
}
