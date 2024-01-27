use tests; // brings the functions we want to test into scope
mod common; // brings the common module into scope
#[test]
fn it_adds_two() {
    common::helper();
    assert_eq!(4, tests::add_two(2));
}
