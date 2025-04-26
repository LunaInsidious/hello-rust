extern crate hello_rust;
mod common;

#[test]
fn it_adds_two() {
  common::setup();
  assert_eq!(4, hello_rust::add_two(2));
}
