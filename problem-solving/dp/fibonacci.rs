//! To run/test the solution please enter the commands below
//! `cargo run --bin fibonacci`
//! `cargo test --bin fibonacci`
use std::io;

fn fib(n: usize) -> usize {
    /*
    Recursive function
    It is much slower since it has overhead of function calling on each recursion.
    if you try using `fib()` function on higher numbers, it may take forever to run.
    */
    return match n {
        1..=2 => 1 as usize,
        _ => fib(n - 1) + fib(n - 2),
    };
}

fn fibonacci(mut n: usize) -> usize {
    /*
    This approach is better than recursive function since it do not
    have any overhead of calling functions on each loop. */
    let mut result = 1;
    let mut a: usize = 1;
    let mut b: usize = 1;
    if n < 2 {
        return result;
    }

    while n > 2 {
        result = a + b;
        a = b;
        b = result;
        n -= 1;
    }
    return result;
}

/**
 * The main function of the program. It prompts the user to enter a number,
 * calculates the nth number in the Fibonacci series using both a recursive and
 * iterative approach, and prints the results.
 *
 * This solution works until 93rd number in the fibonacci series since the
 * solution uses `usize` which is 64 bit unsigned integer.
 *
 * None. The function prints the results to the console.
 */
fn main() {
    let mut input = String::new();
    println!("Enter the value of n: ");
    io::stdin().read_line(&mut input).unwrap();
    //shadowing previous input since it will be useless for us after parsing
    let input = input.trim().parse::<usize>().unwrap();
    println!(
        "The {}th number of the fibonacci series is: {}",
        input,
        fibonacci(input)
    );
    if input < 40 {
        println!(
            "The {}th number of the fibonacci series using recursion is: {}",
            input,
            fib(input)
        );
    } else {
        println!("The recursive calculation is skipped since it is inefficient at higher numbers")
    }
}

#[cfg(test)]
mod tests {
    use crate::{fib, fibonacci};

    #[test]
    fn fib_without_recursion() {
        assert_eq!(fibonacci(10), 55);
    }
    #[test]
    fn fib_with_recursion() {
        assert_eq!(fib(10), 55);
    }

    /// this panics since it is 197,402,742,198,682,231,67 and can not fit into
    /// the `usize` hence it panics with the message:
    /// "attempt to add with overflow"
    #[test]
    #[should_panic]
    fn fib_with_recursion_above_usize() {
        fibonacci(94);
    }
}
