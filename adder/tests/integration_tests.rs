use adder;
mod common;     // Call module from subdirectory

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}
