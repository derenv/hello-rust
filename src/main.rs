// Imports
use rand::Rng;
use std::cmp::Ordering;
use std::io;

/*
 * Title:
 * main
 *
 * Description:
 * This is a function that does x
 *
 * Parameters:
 * none
 *
 * Return Value:
 * none
 */
fn main() {
    // this is a single-line comment

    /*
     * This is a multi-line comment
     */

    // This is how to declare variables
    let x = 5; //immutable: can't be changed, will throw Err if you try
    let mut y = 6; //mutable: can be changed
    const z: i32 = 7; //constant: always immutable
    let q = &x * &y; //x and y are used as references here

    // Printing
    // NOTE: the '!' means this is a macro, not a function
    println!("This is how to print a string");
    println!(
        "This is how to print a string with some inserted var: {}",
        2
    );

    // Input
    let mut guess = String::new();
    // '&mut guess' creates a mutable reference
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line!!!!");
    // IMPORTANT: read_line appends to passed
    // this could have been called as 'std::io::stdin' without the import at line 2
    // guess needs to be stripped of newlines
    let guess: String = guess.trim().to_string();
    println!("You guessed {} dickwad!!", guess);

    let mut guess2: String = String::new();
    guess2 = "fuck you ".to_string(); // text declared in double brackets are str references (like char arrays?)
    io::stdin()
        .read_line(&mut guess2)
        .expect("Failed to read line!!!!");
    let guess2: String = guess2.trim().to_string();
    println!("You guessed {} dickwad!!", guess2);

    // 1..101 and 1..=100 both specify 1-100
    let secret_number1 = rand::thread_rng().gen_range(1..101);
    let secret_number2 = rand::thread_rng().gen_range(1..=100);

    /*
     * Result is an enum basically returned from every operation,
     * ie pass == 'Ok' and fail+why == 'Err'
     */

    // 'Ordering' is an enum
    println!("1: {}", secret_number1);
    println!("2: {}", secret_number2);
    match secret_number1.cmp(&secret_number2) {
        Ordering::Less => println!("1 less than 2"),
        Ordering::Greater => println!("1 more than 2"),
        Ordering::Equal => println!("1 equals 2"),
    }

    // Input a number
    let mut guess3 = String::new();
    let secret_number3 = rand::thread_rng().gen_range(1..101);
    println!("Secret number: {}", secret_number3);
    loop {
        println!("INPUT A NUMBER:");
        io::stdin()
            .read_line(&mut guess3)
            .expect("Failed to read line!!!!");
        println!("You guessed: {}", guess3);
        let guess3: u32 = match guess3.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        //.expect("Please type a number!");// 'shadowing' means we can overwrite defined variables
        //^NOTE: 'parse()' can parse multiple number types, *need* to specify type of output
        match guess3.cmp(&secret_number3) {
            Ordering::Less => println!("guess less than number"),
            Ordering::Greater => println!("guess more than number"),
            Ordering::Equal => {
                println!("guess equals number");
                break;
            }
        }
    }
}
