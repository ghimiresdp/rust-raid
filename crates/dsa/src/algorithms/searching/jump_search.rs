//! # Jump Search
//!
//! To run/test, please run the following commands in your terminal
//!
//! ```sh
//! cargo run --bin jump-search
//! ```
//!
//! ```sh
//! cargo test --bin jump-search
//! ```
//! Jump Searching algorithm uses divide and conquer method to find out elements
//! of a sorted array. This algorithm works by jumping ahead by fixed steps or
//! blocks to find the desired element, and then performing a linear search
//! within the identified block.
//!
//! For an ordered array of size n, the optimal jump size is √n, which minimizes
//! the number of comparisons needed to find the desired element. Hence, the
//! time complexity of jump search is O(√n).
//!
//! Jump search is more efficient than linear search for larger arrays, but less
//! efficient than binary search. It is particularly useful when the cost of
//! jumping ahead is less than the cost of performing multiple comparisons, such
//! as in scenarios involving slow memory access or disk reads.
//!
//! **Note:** Jump search works only on the sorted array. If an array is
//! not sorted, we must sort it before we can implement this algorithm.
//!
//! Example:
//!
//! we have an array `[1, 3, 4, 6, 7, 8, 9, 11, 15]` and we want to find out an
//! index of `7`.
//!
//! ### steps
//! 1. jump ahead by √n (i.e., 3) to index 2, compare the item (4 < 7)
//! 2. jump ahead by √n to index 5, compare the item (8 > 7) (stop here as we have
//!    crossed the desired item)
//! 3. perform linear search from index 3 to 5
//! 4. compare item at index 3 (6 < 7)
//! 5. compare item at index 4 (7 == 7); return index of that number
//!
use std::cmp::Ordering::{Equal, Greater, Less};
use utils::parse_input; // common library for this repository

fn jump_search(array: &mut [i32], item: i32) -> Option<usize> {
    if array.is_empty() {
        return None;
    }

    // step = √n, ensure at least 1
    let step = ((array.len() as f64).sqrt().ceil() as usize).max(1);
    let mut left = 0;
    let mut right = step.min(array.len().saturating_sub(1));

    // item out of bounds
    if item < array[0] || item > array[array.len() - 1] {
        return None;
    }

    // quick check for first element
    if array[left] == item {
        return Some(left);
    }

    while left < array.len() {
        println!("⛔ index [{:2}] to [{:2}]", left, right);
        match array[right].cmp(&item) {
            Greater => {
                println!("🔍 Searching between indexes [{:2}] to [{:2}]", left, right);
                for idx in left..=right {
                    if array[idx] == item {
                        return Some(idx);
                    }
                    if array[idx] > item {
                        return None;
                    }
                }
                return None;
            }
            Equal => return Some(right),
            Less => {
                left = right;
                right = right.saturating_add(step);
                if right > array.len() - 1 {
                    right = array.len() - 1;
                }
            }
        }
    }

    None
}

fn main() {
    let mut sorted_array = [1, 4, 7, 8, 9, 10, 11, 12, 15, 20];
    println!("Sorted Array: {:?}", &sorted_array);
    // ! input() is a common library function, not included in std
    if let Ok(search_item) = parse_input("Enter a number to search: ") {
        return match jump_search(sorted_array.as_mut(), search_item) {
            Some(idx) => println!("The item {} is at index: {}", search_item, idx),
            None => println!("The item {} does not exist in the array", search_item),
        };
    }
    println!("Invalid number Entered")
}
#[cfg(test)]
mod tests {
    use crate::jump_search;

    #[test]
    fn search_ok() {
        assert_eq!(jump_search([1, 4, 2, 5, 7].as_mut(), 5), Some(3))
    }

    #[test]
    fn search_err() {
        assert_eq!(jump_search([1, 4, 2, 5, 7].as_mut(), 8), None)
    }

    // Simple linear congruential generator for deterministic pseudo-random numbers
    fn lcg(seed: &mut u64) -> u64 {
        *seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
        *seed
    }

    fn gen_sorted_array(n: usize, mut seed: u64) -> Vec<i32> {
        let mut v = Vec::with_capacity(n);
        for _ in 0..n {
            let r = lcg(&mut seed);
            // use upper bits to get a 32-bit-ish value
            v.push((r >> 16) as i32);
        }
        v.sort_unstable();
        v
    }

    #[test]
    fn random_present() {
        let sizes = [1, 2, 5, 10, 50, 101, 500];
        for (si, &n) in sizes.iter().enumerate() {
            let seed = 0xDEADBEEF_u64.wrapping_add(si as u64);
            let arr = gen_sorted_array(n, seed);
            if arr.is_empty() {
                continue;
            }
            for pick in 0..arr.len() {
                let key = arr[pick];
                let mut v = arr.clone();
                let res = jump_search(v.as_mut_slice(), key);
                assert!(res.is_some(), "expected to find {} in {:?}", key, v);
                let idx = res.unwrap();
                assert_eq!(v[idx], key);
            }
        }
    }

    #[test]
    fn random_absent() {
        let sizes = [1, 2, 5, 10, 50, 101, 500];
        for (si, &n) in sizes.iter().enumerate() {
            let seed = 0xBEEFDEAD_u64.wrapping_add(si as u64);
            let arr = gen_sorted_array(n, seed);
            if arr.is_empty() {
                continue;
            }
            let mut v = arr.clone();
            // pick a value greater than the max in array
            let last = *v.last().unwrap();
            let key = last.wrapping_add(1);
            assert_eq!(jump_search(v.as_mut_slice(), key), None);
        }
    }
}
