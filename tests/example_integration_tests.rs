// Imports
use hello_rust;

/*
 * Integration tests
 */

// Example
#[test]
fn example_inegration_test() {
    assert_eq!(23, hello_rust::function_for_test2(23));
}

// number generation
#[test]
fn test_generate_number_inclusive() {
    assert_eq!(100, hello_rust::generate_number_inclusive(100, 100));

    let x: i32 = hello_rust::generate_number_inclusive(0, 100);
    assert!(x < 100 && x > 0);
}
#[test]
fn test_generate_number_exclusive() {
    assert_eq!(100, hello_rust::generate_number_exclusive(100, 101));

    let x: i32 = hello_rust::generate_number_exclusive(0, 101);
    assert!(x < 100 && x > 0);
}
