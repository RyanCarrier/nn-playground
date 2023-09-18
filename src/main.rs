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
    // main_1();
    main_2();
}

fn main_1() {
    let test_cases = TestCaseOr::get_all_generic();
    for i in [1000].iter() {
        run_1(*i, &test_cases);
    }
}
fn run_1(iter: usize, test_cases: &Vec<GenericTestCase>) {
    let output_fn: fn(f64) -> f64 = |x| x.min(1.0).max(0.0);
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
    let test_cases = TestCaseOrAnd::get_all_generic();
    let mut network = network::Network::new(3, 1, 4, 2, Some(|x: f64| x.min(1.0).max(0.0)));
    match network.learn(&test_cases, Some(10000), None) {
        Ok(_) => (),
        Err(e) => panic!("{}", e),
    }
    match network.test_all(&test_cases) {
        Ok(result) => {
            println!("{}: {}", 10000, result);
        }
        Err(e) => panic!("{}", e),
    }
}

pub struct GenericTestCase {
    input: Vec<f64>,
    output: Vec<f64>,
    display: String,
}
