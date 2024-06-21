/// # Learning Rust as I go
/// ## WunderVision 2024
/// This project is just about figuring out
/// how to do things in Rust.
/// It is not meant to be an example
/// of good Rust or good code in general.

// How to import other "crates"
use std::{
    io::{self, Write},
    process::exit,
};

// How to make other files a module?
// **To-Do** Look at what this means for large projects
mod number_fun;

// How to import multiple crates
// from the same crate
use number_fun::{
    collatz,
    collatz_loop
};

fn main() {
    // **println** is a "macro" that outputs to the stdout
    println!("Hello, world!");

    // non mutable string "slice"
    // This is different than a **String**
    let output_test = "This is a test output\n";

    // Manually write a string to the console. Note that you need a newline
    // or flush to have it output immediately
    // The return of write_all is a Result "Enum" that can be checked
    // for success.
    // Match is similar in concept to the Switch/Case
    match io::stdout().write_all(output_test.as_bytes()) {
        Ok(_) => {},
        Err(_) => {
            println!("Could not output your string");
        }
    }

    // mutable String, allocated to the heap and can
    // be added to or manipulated
    let mut input = String::new();
    // read in from the stdin using a mutable reference to the input string
    io::stdin()
        .read_line(&mut input)
        .expect("Could not read from stdin for some reason");

    // trim returns a refernce to a string slice
    // this is not a direct copy to the original
    // but a reference to the original string but
    // the start and end differently
    let trimmed_input = input.trim();

    // using parse with the fish operator
    // pattern match the "Result"
    let resulting_int: i32;
    match trimmed_input.parse::<i32>() {
        Ok(parsed_int) => {
            println!("You input an int {}", parsed_int);
            resulting_int = parsed_int;
        },
        Err(..) => {
            println!("This was not an integer!");
            exit(0);
        }
    }

    collatz(resulting_int);
    let mut collatz_collection: Vec<i32> = Vec::new();

    collatz_loop(resulting_int, &mut collatz_collection);

    // Iterating over a vector
    // explit call to iter() required because
    // otherwise implicitly into_iter() is called
    // into_iter() moves the values into a new context
    // and the collatz_collection would become invalid
    for result in collatz_collection.iter() {
        println!("->{}", result);
    }

    if collatz_collection.len() > 2 {
        let slice_of_vect = &collatz_collection[0..2];
        // here slice_of_vect still calls into_iter
        // but it cannot give ownership away since itself
        // is a slice to the underliying collatz_collection
        for slice_value in slice_of_vect {
            println!("slice value {}", slice_value);
        }
        let _ = slice_of_vect.ends_with(&[4]);
    }
}
