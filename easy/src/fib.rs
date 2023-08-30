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
    // too slower for higher numbers
    // println!(
    //     "The {}th number of the fibonacci series is: {}",
    //     input,
    //     fib(input)
    // );
}
