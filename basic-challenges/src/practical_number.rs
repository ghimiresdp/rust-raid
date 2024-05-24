use std::io;

fn divisors(num: u32) -> Vec<u32> {
    let mut divisors: Vec<u32> = Vec::new();
    for n in 1..=num / 2 {
        if num % n == 0 {
            divisors.push(n);
        }
    }
    return divisors;
}

fn is_practical_number(num: u32) -> bool {
    divisors(num).iter().sum::<u32>() == num
}

fn main() {
    let mut input = String::new();
    println!("Input a number: ");
    io::stdin().read_line(&mut input).unwrap();

    println!("You have entered: {}", input);
    let number = input.trim().parse::<u32>().unwrap();
    println!(
        "The number {} is{}a practical Number",
        number,
        if is_practical_number(number) {
            " "
        } else {
            " not "
        }
    )
}

#[cfg(test)]
mod tests {
    use crate::{divisors, is_practical_number};

    #[test]
    fn successful_divisors() {
        assert_eq!(divisors(6), vec![1, 2, 3]);
        assert_eq!(divisors(12), vec![1, 2, 3, 4, 6]);
    }

    #[test]
    fn practical_numbers() {
        for x in [6] {
            // Example:  factorials of 6 are (1, 2, 3) => 1 + 2 + 3 = 6
            assert_eq!(is_practical_number(x), true);
        }
    }

    #[test]
    fn non_practical_numbers() {
        for x in [5, 10, 12]{
            assert_eq!(is_practical_number(x), false);
        }
    }
}
