extern crate ch1103_test_organization;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, ch1103_test_organization::add_two(2));
}
