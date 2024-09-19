/**
 * -----------------------------------------------------------------------------
 * FIND THE MISSING NUMBER
 *
 * To execute, please run: cargo run --bin ds001
 * To run tests, please run: cargo test --bin ds001
 * -----------------------------------------------------------------------------
 *
 * Write a program (and a test case) that finds a missing number from an array
 * containing distinct numbers.
 *
 * CONSTRAINTS
 * - The vector should have exactly 1 missing number.
 * - The vector should have n distinct 32 bit integers  (i32).
 * - The function should return the missing number as Option<i32>.
 * - If there are no missing numbers, the function should return None.
 *
 * EXAMPLE:
 * - input: [3, 7, 1, 2, 8, 4, 5]
 * - output: 6
 */

/**
 * Approach 1: Using sorting.
 *
 * Sorting the vector will have the time complexity of O(n log n)
 * We first sort the number and
 */
fn find_missing_number(mut num: Vec<i32>) -> Option<i32> {
    if num.len() < 2 {
        return None;
    }
    num.sort();
    for i in 0..num.len() - 1 {
        if num[i + 1] - num[i] != 1 {
            return Some(num[i] + 1);
        }
    }
    None
}

/**
 * Approach 2: calculating the expected sum and returning the difference
 *
 * This approach is more efficient since this approach has the time
 * complexity of exactly O(n).
 */
fn find_missing_num(num: Vec<i32>) -> Option<i32> {
    let smallest = num.iter().min().unwrap();
    let largest = num.iter().max().unwrap();
    let sum: i32 = num.iter().sum();

    let expected_sum = ((largest * (largest + 1)) - (smallest * (smallest - 1))) / 2;
    match expected_sum - sum {
        0 => None,
        s => Some(s),
    }
}

fn main() {
    let numbers = vec![3, 7, 1, 2, 8, 4, 5];
    println!(
        "missing number in an array {:?} is: {}",
        numbers.clone(),
        find_missing_number(numbers.clone()).unwrap_or(-1)
    );
    println!(
        "missing number in an array {:?} is: {}",
        numbers.clone(),
        find_missing_num(numbers).unwrap_or(-1)
    )
}

#[cfg(test)]
mod tests {
    use crate::{find_missing_num, find_missing_number};

    #[test]
    fn contains_missing() {
        assert_eq!(find_missing_number(vec![3, 7, 1, 2, 8, 4, 5]), Some(6));
        assert_eq!(find_missing_num(vec![3, 7, 1, 2, 8, 4, 5]), Some(6));
    }
    #[test]
    fn contains_no_missing() {
        assert_eq!(find_missing_number(vec![3, 7, 1, 2, 8, 4, 5, 6]), None);
        assert_eq!(find_missing_num(vec![3, 7, 1, 2, 8, 4, 5, 6]), None);
    }
    #[test]
    fn contains_no_missing_when_one_element() {
        assert_eq!(find_missing_number(vec![1]), None);
        assert_eq!(find_missing_num(vec![1]), None);
    }
}
