use criterion::{black_box, criterion_group, criterion_main, Criterion};
use nn_playground::{
    cases::simple::or_and::TestCaseOrAnd,
    networks::{network1::network::Network1, network2::network::Network2},
    traits::{generic_test_case::GenericTestCase, network_traits::BaseNetwork},
};
const INTERNAL_LAYERS: usize = 2;
const INTERNAL_NODES: usize = 4;

fn get_network1() -> Network1 {
    Network1::new(3, 1, INTERNAL_NODES, INTERNAL_LAYERS, None)
}
fn get_network2() -> Network2 {
    Network2::new(3, 1, INTERNAL_NODES, INTERNAL_LAYERS, None)
}
fn learn_cases(
    network: &mut impl BaseNetwork,
    test_cases: &Vec<GenericTestCase<Vec<f64>, f64>>,
) -> Result<Vec<f64>, String> {
    network.learn(&test_cases, Some(10_000), None)
}

fn criterion_benchmark(c: &mut Criterion) {
    let network1 = get_network1();
    let test_cases = TestCaseOrAnd::get_all_generic();
    let n1result = learn_cases(&mut network1.clone(), &test_cases);
    assert!(n1result.is_ok(), "n1, Error: {}", n1result.unwrap_err());
    let n1errors = n1result.unwrap();
    let final_err = n1errors.last().unwrap();
    assert_eq!(*final_err, 0.0, "n1, result: {:?}", final_err);
    c.bench_function(&network1.title(), |b| {
        b.iter(|| learn_cases(black_box(&mut network1.clone()), black_box(&test_cases)))
    });
    let network2 = get_network2();
    let n2result = learn_cases(&mut network2.clone(), &test_cases);
    assert!(n2result.is_ok(), "n2, Error: {}", n2result.unwrap_err());
    let n2errors = n2result.unwrap();
    let final_err = n2errors.last().unwrap();
    assert_eq!(*final_err, 0.0, "n2, result: {:?}", final_err);
    c.bench_function(&network2.title(), |b| {
        b.iter(|| learn_cases(black_box(&mut network2.clone()), black_box(&test_cases)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
