use generic_test_case::GenericTestCase;

mod cases;
mod generic_test_case;
mod network1;
mod network_traits;
mod run;
mod run_game;

fn main() {
    // cases::simple::runner();
    cases::game::runner();
}
