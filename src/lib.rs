use rand::Rng;
/*
 * Title:
 * function_for_test2
 *
 * Description:
 * Example function for integration tests
 *
 * Parameters:
 * x: i32
 *
 * Return Value:
 * i32
 */
pub fn function_for_test2(x: i32) -> i32 {
    x
}

/*
 * Title:
 * generate_number_inclusive
 *
 * Description:
 * Generate a number from x to y (inclusive of y)
 *
 * Parameters:
 * x: i32
 * y: i32
 *
 * Return Value:
 * i32
 */
pub fn generate_number_inclusive(x: i32, y: i32) -> i32 {
    rand::thread_rng().gen_range(x..=y)
}

/*
 * Title:
 * generate_number_exclusive
 *
 * Description:
 * Generate a number from x to y (exclusive of y)
 *
 * Parameters:
 * x: i32
 * y: i32
 *
 * Return Value:
 * i32
 */
pub fn generate_number_exclusive(x: i32, y: i32) -> i32 {
    rand::thread_rng().gen_range(x..y)
}

/*
 * Title:
 * printing
 *
 * Description:
 * Print input string with any params
 *
 * Also demonstrates None/Some matching
 *
 * Parameters:
 * input_string: &str
 * param: String
 *
 * Return Value:
 * none
 */
pub fn printing(input_string: &str, param: Option<String>) {
    // NOTE: the '!' means this is a macro, not a function
    match param {
        None => {
            println!("{}", input_string);
        }
        Some(nonnull_param) => {
            println!("{}{}", input_string, nonnull_param);
        }
    }
}
