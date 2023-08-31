/**
 * Greatest Common Divisor (GCD) or Highest Common Factor (HCF)
 *
 * The task is to find the GCD among  numbers input in the console
 */
use _lib::input;

fn divisors(num: usize) -> Vec<usize> {
    let mut divisors: Vec<usize> = Vec::new();
    for n in 1..=num / 2 {
        if num % n == 0 {
            divisors.push(n);
        }
    }
    divisors.push(num);

    return divisors;
}

fn gcd(mut list: Vec<usize>) -> usize {
    list.sort();

    // removing smallest of list to find divisors
    let smallest = list.remove(0);
    if smallest == 0 {
        return 0;
    };

    // finding possible divisors of smallest number
    let mut possible_divisors = divisors(smallest);

    possible_divisors.sort();
    possible_divisors.reverse();

    for div in possible_divisors {
        if list.iter().filter(|&f| f % div == 0).count() == list.len() {
            return div;
        };
    }

    return 1;
}
fn main() {
    let numbers: Vec<usize> = input("Enter numbers separated by space")
        .trim()
        .split(" ")
        .map(|v| v.parse::<usize>().unwrap())
        .collect();

    println!("The GCD of {:?} is {}", numbers.clone(), gcd(numbers));
}

#[cfg(test)]
mod test {
    use crate::gcd;

    #[test]
    fn _2_numbers() {
        assert_eq!(gcd(vec![12, 18]), 6)
    }

    #[test]
    fn _3_numbers() {
        assert_eq!(gcd(vec![12, 18, 9]), 3)
    }

    #[test]
    fn zero() {
        assert_eq!(gcd(vec![12, 18, 9, 0]), 0)
    }
}
