use std::io::{self, Write};

pub fn input(prompt: &str) -> String {
    /**
     * It takes a parameter prompt as a &str and prints out as a prompt before
     * taking input from a user. This method is similar to input() method in
     * python.
     */
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut _str = String::new();
    io::stdin().read_line(&mut _str).unwrap();
    _str
}
