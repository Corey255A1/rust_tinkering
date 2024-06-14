// How to import other "crates"
use std::{
    io::{self, Write},
    process::exit,
};

fn main() {
    // println is a "macro" that outputs to the stdout
    println!("Hello, world!");

    // non mutable string "slice"
    let output_test = "This is a test output\n";

    // Manually write a string to the console. Note that you need a newline
    // or flush to have it output immediately
    match io::stdout().write_all(output_test.as_bytes()) {
        Ok(_) => {}
        Err(_) => {
            println!("Could not output your string");
        }
    }

    // mutable string
    let mut input = String::new();
    // read in from the stdin using a mutable reference to the input string
    io::stdin()
        .read_line(&mut input)
        .expect("Could not read from stdin for some reason");

    // trim returns a refernce to a string slice
    // this is not a direct copy to the original.
    let trimmed_input = input.trim();

    // using parse with the fish operator
    // pattern match the "Result"
    let mut resulting_int: i32 = 0;
    match trimmed_input.parse::<i32>() {
        Ok(parsed_int) => {
            println!("You input an int {}", parsed_int);
            resulting_int = parsed_int;
        }
        Err(..) => {
            println!("This was not an integer!");
            exit(0);
        }
    }

    collatz(resulting_int);
}

fn collatz(n: i32) -> i32 {
    println!("{}",n);
    match n {
        1 => return 1,
        x if x & 1 == 1 => return collatz(3 * n + 1),
        _ => return collatz(n / 2),
    }
}
