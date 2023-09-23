use crate::{run, GenericTestCase};

pub fn runner() {
    let test_cases = TestCaseTikTakToes::get_all_generic();
    run::run("Game, TikTakToes", &test_cases, 1..5, 1..7);
}

#[cfg(test)]
mod tests {

    use crate::{network1::network, network_traits::BaseNetwork};

    use super::TestCaseTikTakToes;

    fn default_network() -> network::Network1 {
        network::Network1::new(3, 1, 4, 1, None)
    }

    #[test]
    fn learn() {
        let test_cases = TestCaseTikTakToes::get_all_generic();
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
        let error = network.test_all(&TestCaseTikTakToes::get_all_generic());
        assert!(error.is_ok());
        assert_eq!(error.unwrap(), 0.0);
    }
}
#[derive(Clone, Copy)]
pub struct TestCaseTikTakToes {
    pub input: [f64; 3],
    pub output: f64,
}
impl TestCaseTikTakToes {
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
        TestCaseTikTakToes::get_all()
            .iter()
            .map(|x| x.to_generic())
            .collect()
    }
    pub fn get_all() -> [TestCaseTikTakToes; 8] {
        let result: [TestCaseTikTakToes; 8] = [TestCaseTikTakToes {
            input: [0.0, 0.0, 0.0],
            output: 0.0,
        }; 8];
        result
    }
}
