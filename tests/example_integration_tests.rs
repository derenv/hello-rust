// Imports
use hello_rust;

/*
 * Integration tests
 */
#[test]
fn example_inegration_test(){
    let y: i32 = 23;
    assert_eq!(23,hello_rust::function_for_test2(y));
}
