use crate::{networks::Networks, run, traits::generic_test_case::GenericTestCase};

pub fn runner(network: &Option<Networks>) {
    let test_cases = TestCaseOrAnd::get_all_generic();
    let layers = 2..5;
    let nodes = 4..7;
    match network {
        Some(Networks::Network1) => {
            run::run("OrAnd", Networks::Network1, &test_cases, layers, nodes)
        }
        Some(Networks::Network2) => {
            run::run("OrAnd", Networks::Network2, &test_cases, layers, nodes)
        }
        Some(Networks::Network3) => {
            run::run("OrAnd", Networks::Network3, &test_cases, layers, nodes)
        }
        None => {
            run::run(
                "OrAnd",
                Networks::Network1,
                &test_cases,
                layers.clone(),
                nodes.clone(),
            );
            run::run("OrAnd", Networks::Network2, &test_cases, layers, nodes);
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        networks::{network1::network::Network1, network2::network::Network2},
        traits::network_traits::BaseNetwork,
    };

    use super::TestCaseOrAnd;
    fn get_network1() -> Network1 {
        Network1::new(3, 1, 4, 1, None)
    }
    fn get_network2() -> Network2 {
        Network2::new(3, 1, 4, 1, None)
    }

    #[test]
    fn learn() {
        let test_cases = TestCaseOrAnd::get_all_generic();
        for _ in 0..20 {
            let mut network = get_network1();
            match network.learn(&test_cases, Some(100_000), None) {
                Ok(err_history) => println!("err_history length: {}", err_history.len()),
                Err(e) => panic!("{}", e),
            }
            test(network);
            let mut network = get_network2();
            match network.learn(&test_cases, Some(100_000), None) {
                Ok(err_history) => println!("err_history length: {}", err_history.len()),
                Err(e) => panic!("{}", e),
            }
            test(network);
        }
    }

    fn test(mut network: impl BaseNetwork) {
        let error = network.test_all(&TestCaseOrAnd::get_all_generic());
        assert!(error.is_ok());
        assert_eq!(error.unwrap(), 0.0, "network: {}", network.title());
    }
}
#[derive(Clone, Copy)]
pub struct TestCaseOrAnd {
    pub input: [f64; 3],
    pub output: f64,
}
impl TestCaseOrAnd {
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
        TestCaseOrAnd::get_all()
            .iter()
            .map(|x| x.to_generic())
            .collect()
    }
    pub fn get_all() -> [TestCaseOrAnd; 8] {
        let mut result: [TestCaseOrAnd; 8] = [TestCaseOrAnd {
            input: [0.0, 0.0, 0.0],
            output: 0.0,
        }; 8];
        //k=0 or, k=1 and
        //haha yeah nice
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    result[i + (2 * j) + (4 * k)] = TestCaseOrAnd {
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
