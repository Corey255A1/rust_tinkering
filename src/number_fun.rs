/// # Learning Rust as I go
/// ## WunderVision 2024
// Trait which is similar to an interface
pub trait NumberProperties {
    fn is_odd(&self) -> bool;
}

// implementing a trait for the built in
// i32 type.. now is_odd can be used
// on a regular built in integer
impl NumberProperties for i32 {
    fn is_odd(&self) -> bool {
        self & 1 == 1
    }
}

// Recursively calling and printing the collatz sequence
pub fn collatz(n: i32) -> i32 {
    println!("{}", n);
    match n {
        1 => return 1,
        // using a trait to replace this if x & 1 == 1 => return collatz(3 * n + 1),
        x if x.is_odd() => return collatz(3 * n + 1),
        _ => return collatz(n / 2),
    }
}

// Passing in a mutable vector and filling it with the results of the collatz loop
pub fn collatz_loop(n: i32, result: &mut Vec<i32>) {
    let mut working = n;
    while working != 1 {
        working = match working {
            x if x.is_odd() => 3 * x + 1,
            _ => working / 2,
        };
        result.push(working);
    }
}