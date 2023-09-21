use generic_test_case::GenericTestCase;

mod generic_test_case;
mod network1;
mod pt1;
mod pt1_5;
mod pt2;
mod pt2_5;
mod pt3;
mod run;
mod run_game;

fn main() {
    pt1::runner();
    pt1_5::runner();
    pt2::runner();
    pt2_5::runner();
    pt3::runner();
}
