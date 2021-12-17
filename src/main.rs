// Imports
use hello_rust;
use std::cmp::Ordering;
use std::io;

/*
 * Title:
 * main
 *
 * Description:
 * This is the main function, calls the other functions
 *
 * Parameters:
 * none
 *
 * Return Value:
 * none
 */
fn main() {
    function_for_test1(0);

    variables(0, 0);

    string_input();

    number_matching(
        hello_rust::generate_number_inclusive(1, 100),
        hello_rust::generate_number_inclusive(1, 100),
    );

    string_matching_eq("".to_string(), "".to_string());
    string_matching_in("onete".to_string(), "te".to_string());

    bc_loop(0, 10);

    for_loop(0, 10);

    while_loop(0, 10);
}

/*
 * Title:
 * function_for_test1
 *
 * Description:
 * basic unit test example
 *
 * Parameters:
 * x: i32
 *
 * Return Value:
 * i32
 */
fn function_for_test1(x: i32) -> i32 {
    x
}

/*
 * Title:
 * variables
 *
 * Description:
 * Create different types of variable
 *
 * preceding underscore means the variable is never used
 *
 * Parameters:
 * none
 *
 * Return Value:
 * none
 */
fn variables(s: i32, t: i32) -> i32 {
    // This is how to declare variables
    let _x: i32 = 5; //immutable: can't be changed, will throw Err if you try
    let mut _y: i32 = 6; //mutable: can be changed
    _y *= 9; //compound operators! yay!
    const _Z: i32 = 7; //constant: always immutable
    let q: i32 = s * t; // using variables

    q
}

/*
 * Title:
 * string_input
 *
 * Description:
 * Get some string input and print
 *
 * Take some string and trim newlines off
 *
 * also meant to example Result, an enum basically returned from every operation:
 * ie pass == 'Ok' and fail+reason == 'Err'
 *
 * Parameters:
 * none
 *
 * Return Value:
 * none
 */
fn string_input() -> String {
    // Input variable
    let mut some_input = String::new();

    // '&mut guess' creates a mutable reference
    // IMPORTANT: read_line appends to passed
    // this could have been called as 'std::io::stdin' without the import at line 2
    io::stdin()
        .read_line(&mut some_input)
        .expect("Failed to read line!!!!");

    // guess needs to be stripped of newlines
    some_input.trim().to_string()
}

/*
 * Title:
 * number_matching
 *
 * Description:
 * match statements comparing numbers to example 'Ordering', an enum with 3 possible values
 *
 * Parameters:
 * x: i32
 * y: i32
 *
 * Return Value:
 * bool
 */
fn number_matching(x: i32, y: i32) -> Ordering {
    hello_rust::printing("x: ", Some(x.to_string()));
    hello_rust::printing("y: ", Some(y.to_string()));
    x.cmp(&y)
}

/*
 * Title:
 * string_matching
 *
 * Description:
 * check if strings match
 *
 * Parameters:
 * x: String
 * y: String
 *
 * Return Value:
 * bool
 */
fn string_matching_eq(x: String, y: String) -> bool {
    // Compare strings
    x.eq(&y)
}

/*
 * Title:
 * string_matching_in
 *
 * Description:
 * check if one string is substring of another
 *
 * Parameters:
 * x: String
 * y: String
 *
 * Return Value:
 * bool
 */
fn string_matching_in(x: String, y: String) -> bool {
    // Compare strings
    x.contains(&y)
}

/*
 * Title:
 * bc_loop
 *
 * Description:
 * loop using continue until break occurs
 *
 * Parameters:
 * x: i32
 * y: i32
 *
 * Return Value:
 * i32
 */
fn bc_loop(mut x: i32, y: i32) -> i32 {
    let mut loops: i32 = 0;
    loop {
        if x == y {
            break;
        } else {
            x += 1;
            loops += 1;
            continue;
        }
    }

    loops
}

/*
 * Title:
 * for_loop
 *
 * Description:
 * loop until condition met
 *
 * Parameters:
 * x: i32
 * y: i32
 *
 * Return Value:
 * i32
 */
fn for_loop(x: i32, y: i32) -> i32 {
    let mut loops: i32 = 0;
    for z in x..y {
        println!("{}", z);
        loops += 1;
    }

    loops
}

/*
 * Title:
 * while_loop
 *
 * Description:
 * loop while condition met
 *
 * Parameters:
 * x: i32
 * y: i32
 *
 * Return Value:
 * i32
 */
fn while_loop(mut x: i32, y: i32) -> i32 {
    let mut loops: i32 = 0;
    while x < y {
        x += 1;
        loops += 1;
    }

    loops
}

/*
 * Unit tests
 */
#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::Ordering;

    // Basic unit test example
    #[test]
    fn unit_test_1() {
        let y: i32 = 23;
        assert_eq!(23, function_for_test1(y));
        println!("test runs OK");
    }

    // variables
    #[test]
    fn unit_test_variables() {
        assert_eq!(1, variables(1, 1));
        assert_eq!(8, variables(2, 4));
        assert_eq!(0, variables(0, 4));
        assert_eq!(0, variables(2, 0));
        assert_eq!(0, variables(0, 0));
        println!("test runs OK");
    }

    /*
     * string input
     *
     * this kind of test is hard to write.. you would generally fake such an input lol
     */
    //#[test]
    //fn unit_test_string_input() {
    //    string_input();
    //    assert_eq!("fuck", string_input());
    //}

    // number comparison
    #[test]
    fn unit_test_number_matching() {
        assert_eq!(Ordering::Less, number_matching(1, 2));
        assert_eq!(Ordering::Greater, number_matching(2, 1));
        assert_eq!(Ordering::Equal, number_matching(1, 1));
    }

    // string comparison 1
    #[test]
    fn unit_test_string_matching_eq() {
        assert_eq!(
            true,
            string_matching_eq("one".to_string(), "one".to_string())
        );
        assert_eq!(
            false,
            string_matching_eq("two".to_string(), "one".to_string())
        );
    }

    // string comparison 2
    #[test]
    fn unit_test_string_matching_in() {
        assert_eq!(
            true,
            string_matching_in("one".to_string(), "one".to_string())
        );
        assert_eq!(
            true,
            string_matching_in("onetwo".to_string(), "one".to_string())
        );
        assert_eq!(
            false,
            string_matching_in("onetwo".to_string(), "three".to_string())
        );
    }

    // break/continue loop until
    #[test]
    fn unit_test_bc_loop() {
        assert_eq!(10, bc_loop(0, 10));
    }

    // for loop until
    #[test]
    fn unit_test_for_loop() {
        assert_eq!(10, for_loop(0, 10));
    }

    // for loop until
    #[test]
    fn unit_test_while_loop() {
        assert_eq!(10, while_loop(0, 10));
    }
}
