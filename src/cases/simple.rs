pub mod and;
pub mod or;
pub mod or_and;

pub fn runner() {
    or::runner();
    and::runner();
    or_and::runner();
}
