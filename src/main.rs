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
    for i in [10, 10, 10].iter() {
        let mut network = network::Network::new(2, 1, 3, 3);
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
fn main_2() {
    let test_cases = TestCaseOrAnd::get_all()
        .iter()
        .map(|x| x.to_generic())
        .collect();
    for i in [10, 1000, 100000].iter() {
        let mut network = network::Network::new(2, 1, 2, 1);
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
    use crate::network;
    use crate::TestCaseOr;

    #[test]
    fn pt1() {
        let test_cases = TestCaseOr::get_all()
            .iter()
            .map(|x| x.to_generic())
            .collect();
        for i in [10, 1000, 100000].iter() {
            let mut network = network::Network::new(2, 1, 2, 1);
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
}
pub struct GenericTestCase {
    input: Vec<f64>,
    output: Vec<f64>,
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
        }
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
                output: 0.0,
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
        }
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
