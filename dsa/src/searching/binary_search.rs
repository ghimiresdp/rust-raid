use _lib::input;
use std::cmp::Ordering::{Equal, Greater, Less};
/**
* Binary Search
*
* Binary Searching algorithm uses divide and conquer method to recursively find
* out elements of the item.
*
* It first compares middle most item of an array and recursively divides the
* array into sub-arrays virtually until it finds out the exact element.
*
* It works recursively and non-recursively, but as recursive functions are not
* good for larger arrays, we implemented while loop.
*
* REMEMBER!!: The binary search works only on the sorted array. If an array is
* not sorted, we must sort it before we implement this algorithm.
*
* Example,
*
* we have an array [1, 3, 4, 6, 7, 8, 9] and we want to find out an index of 7
*
* step 1: compare the middle most item with 7 [6 < 7]
* step 2: split an array and take the right slice [7 8 9] (take left if the comparison was different)
* step 3: compare the middle most item of [7 8 9], i.e [8 > 7] (take the left slice)
* step 4: compare the middle most item fo [7]: i.e [7 == 7]; return index of that number

**/

fn binary_search(array: &mut [i32], item: i32) -> Option<usize> {
    array.sort();
    println!("Sorted array is: {:?}", array);
    let mut left = 0;
    let mut right = array.len() - 1;

    while left <= right {
        let mid = (left + right) / 2;
        match array[mid].cmp(&item) {
            Less => left = mid + 1,
            Equal => return Some(mid),
            Greater => right = mid - 1,
        }
    }
    return None;
}

fn main() {
    let mut arr_1 = [1, 4, 7, 8, 9, 10, 11, 12, 15, 20];
    let search_item = input("Enter a number to search")
        .trim()
        .parse::<i32>()
        .unwrap();
    match binary_search(arr_1.as_mut(), search_item) {
        Some(idx) => println!("The item {} is at index: {}", search_item, idx),
        None => println!("The item {} does not exist in the array", search_item),
    }
}
#[cfg(test)]
mod tests {
    use crate::binary_search;

    #[test]
    fn search_ok() {
        assert_eq!(binary_search([1, 4, 2, 5, 7].as_mut(), 5), Some(3))
    }

    #[test]
    fn search_err() {
        assert_eq!(binary_search([1, 4, 2, 5, 7].as_mut(), 8), None)
    }
}
