use pt1::TestCaseOr;
use pt2::TestCaseOrAnd;

pub mod layer;
mod network;
pub mod node;
pub mod path;
mod pt1;
mod pt1_5;
mod pt2;

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
    match network.learn(&test_cases, Some(iter), None) {
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
        match network.learn(&test_cases, Some(*i), None) {
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

pub struct GenericTestCase {
    input: Vec<f64>,
    output: Vec<f64>,
    display: String,
}
