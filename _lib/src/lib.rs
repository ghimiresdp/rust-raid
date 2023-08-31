use std::io;

pub fn input(prompt: &str) -> String{
    println!("{}", prompt);
    let mut _str = String::new();
    io::stdin().read_line(&mut _str).unwrap();
    _str
}
