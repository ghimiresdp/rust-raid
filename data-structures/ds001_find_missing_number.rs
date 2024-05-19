/**
 * Find the missing number
 *
 * Write a program (and a test case) that finds a missing number from an array
 * containing distinct numbers.
 *
 * CONSTRAINTS
 * - If the array has more than 1 missing numbers, return the smallest one.
 * - The array might have n distinct 32 bit integers  (i32).
 * - The function should return the missing number as Option<i32>.
 * - If there are no missing numbers, the function should return None.
 *
 * EXAMPLE:
 * - input: [3, 7, 1, 2, 8, 4, 5]
 * - output: 6
 */

fn find_missing_number(mut num: Vec<i32>) -> Option<i32> {
    // first sort the number
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

fn main() {
    let numbers = vec![3, 7, 1, 2, 8, 4, 5];
    println!(
        "missing number in an array {:?} is: {}",
        numbers.clone(),
        find_missing_number(numbers).unwrap_or(-1)
    )
}

#[cfg(test)]
mod tests {
    use crate::find_missing_number;

    #[test]
    fn contains_missing() {
        assert_eq!(find_missing_number(vec![3, 7, 1, 2, 8, 4, 5]), Some(6));
    }
    #[test]
    fn contains_no_missing() {
        assert_eq!(find_missing_number(vec![3, 7, 1, 2, 8, 4, 5, 6]), None);
    }
    #[test]
    fn contains_no_missing_when_one_element() {
        assert_eq!(find_missing_number(vec![1]), None);
    }
}
