// How to import other "crates"
use std::{
    io::{self, Write},
    process::exit, vec,
};

pub trait NumberProperties {
    fn is_odd(&self) -> bool;
}

impl NumberProperties for i32 {
    fn is_odd(&self) -> bool {
        self & 1 == 1
    }
}

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
    let mut collatz_collection: Vec<i32> = Vec::new();

    collatz_loop(resulting_int, &mut collatz_collection);

    for result in collatz_collection {
        println!("{}", result);
    }

}

// Recursively calling and printing the collatz sequence
fn collatz(n: i32) -> i32 {
    println!("{}", n);
    match n {
        1 => return 1,
        // using a trait to replace this if x & 1 == 1 => return collatz(3 * n + 1),
        x if x.is_odd() => return collatz(3 * n + 1),
        _ => return collatz(n / 2),
    }
}

// Passing in a mutable vector and filling it with the results of the collatz loop
fn collatz_loop(n:i32, result:&mut Vec<i32>) {
    let mut working = n;
    while working != 1 {
        working = match working {
            x if x.is_odd() => 3 * x + 1,
            _ => working / 2,
        };
        result.push(working);
    }
}
