//! # Interpolation Search
//!
//! To run/test, please run the following commands in your terminal
//!
//! ```sh
//! cargo run --bin interpolation-search
//! ```
//!
//! ```sh
//! cargo test --bin interpolation-search
//! ```
//!
//! Interpolation search is an improvement of a binary search algorithm that
//! uses a more efficient way to find the desired element in a sorted array.
//!
//! Instead of checking the middle element, interpolation search uses a linear
//! interpolation formula with the help of probe elements to find the next
//! position to be checked.
//!
//! In a uniformly distributed sorted array, the time complexity of interpolation
//! search is O(log log n), making it faster than binary search's O(log n).
//!
//! Example:
//!
//! we have an array `[1, 3, 4, 6, 7, 8, 9, 11, 15]` and we want to find out an
//! index of `7`.
//!
//! ### Interpolation Formula
//!
//! The position to be probed is calculated using the formula:
//!
//! ```
//! pos = low + ((item - array[low]) * (high - low)) / (array[high] - array[low])
//! ```
//!
//!
//! ### steps
//! 1. calculate the probe position using interpolation formula
//! 2. compare the item at probe position with the desired item
//! 3. if the item matches, return the index
//! 4. if the item is less than the desired item, repeat the process in the
//!    right sub-array, otherwise, repeat in the left sub-array.
//! 5. if the item is not found, return None
//!
use std::cmp::Ordering::{Equal, Greater, Less};
use utils::parse_input; // common library for this repository

fn interpolation_search(
    array: &Vec<i32>,
    low: Option<usize>,
    high: Option<usize>,
    item: i32,
) -> Option<usize> {
    if array.is_empty() {
        return None;
    }

    // initially, probe the entire array if probe is not provided
    let (Some(mut low), Some(mut high)) = (low, high) else {
        return interpolation_search(array, Some(0), Some(array.len() - 1), item);
    };

    // check if the item is within the range of the array
    // `high` and `low` are usize types, so they do not need to be checked for
    // negative values
    if high >= array.len() || low >= array.len() {
        return None;
    }

    // if the item is out of the range of the current sub-array
    // this also avoids division by zero in the interpolation formula
    if array[high] == array[low] && array[low] != item {
        return None;
    }
    let position = low as i32
        + ((item as i32 - array[low]) * (high - low) as i32) / (array[high] - array[low]);

    if position < 0 || position as usize >= array.len() {
        return None;
    }
    match array[position as usize].cmp(&item) {
        Less => {
            low = position as usize + 1;
        }
        Greater => {
            if position as usize == 0 {
                return None;
            }
            high = position as usize - 1;
        }
        Equal => return Some(position as usize),
    }
    return interpolation_search(array, Some(low), Some(high), item);
}

fn main() {
    let sorted_array = vec![1, 4, 7, 8, 9, 10, 11, 12, 15, 20];
    println!("Sorted Array: {:?}", &sorted_array);
    // ! input() is a common library function, not included in std
    if let Ok(search_item) = parse_input("Enter a number to search: ") {
        return match interpolation_search(&sorted_array, None, None, search_item) {
            Some(idx) => println!("The item {} is at index: {}", search_item, idx),
            None => println!("The item {} does not exist in the array", search_item),
        };
    }
    println!("Invalid number Entered")
}
#[cfg(test)]
mod tests {
    use crate::interpolation_search;

    #[test]
    fn search_ok() {
        assert_eq!(
            interpolation_search(&vec![1, 4, 2, 5, 7], None, None, 5),
            Some(3)
        )
    }

    #[test]
    fn search_err() {
        assert_eq!(
            interpolation_search(&vec![1, 4, 2, 5, 7], None, None, 8),
            None
        )
    }
}
