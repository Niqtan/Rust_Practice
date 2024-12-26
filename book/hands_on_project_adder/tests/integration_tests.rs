use adder::add_two;

mod common;
//Looks for the content of the module

#[test]
fn it_adds_two() {
    common::setup();
    let result = add_two(2);
    assert_eq!(result, 4);
}