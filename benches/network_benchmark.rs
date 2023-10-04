use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use nn_playground::{
    cases::simple::or_and::TestCaseOrAnd,
    networks::{network1::network::Network1, network2::network::Network2},
    traits::{generic_test_case::GenericTestCase, network_traits::BaseNetwork},
};
fn learn_cases(
    network: &mut impl BaseNetwork,
    test_cases: &Vec<GenericTestCase<Vec<f64>, f64>>,
) -> Result<Vec<f64>, String> {
    network.learn(&test_cases, Some(10_000), None)
}

fn ensure_correct(
    network: &mut impl BaseNetwork,
    test_cases: &Vec<GenericTestCase<Vec<f64>, f64>>,
) {
    let result = learn_cases(&mut network.clone(), &test_cases);
    assert!(
        result.is_ok(),
        "{}, Error: {}",
        network.title(),
        result.unwrap_err()
    );
    let errors = result.unwrap();
    let final_err = errors.last().unwrap();
    assert_eq!(
        *final_err,
        0.0,
        "{}, result: {:?}",
        network.title(),
        final_err
    );
}

fn or_and(c: &mut Criterion) {
    for i in 2..=5 {
        or_and_internal(c, 2_usize.pow(i));
    }
}
fn or_and_internal(c: &mut Criterion, nodes_layers: usize) {
    let title = String::from("OrAnd ");
    let nodes_title = format!("{}Nodes", nodes_layers);
    let test_cases = TestCaseOrAnd::get_all_generic();
    //network1

    let mut group = c.benchmark_group(title);
    group.bench_function(BenchmarkId::new("Network1", nodes_title.clone()), |b| {
        let network1 = Network1::new(3, 1, nodes_layers, nodes_layers, None);
        // ensure_correct(&mut network1.clone(), &test_cases);
        b.iter(|| learn_cases(black_box(&mut network1.clone()), black_box(&test_cases)))
    });
    //network2
    group.bench_function(BenchmarkId::new("Network2", nodes_title), |b| {
        let network2 = Network2::new(3, 1, nodes_layers, nodes_layers, None);
        // ensure_correct(&mut network2.clone(), &test_cases);
        b.iter(|| learn_cases(black_box(&mut network2.clone()), black_box(&test_cases)))
    });
}

criterion_group!(bench, or_and);
criterion_main!(bench);