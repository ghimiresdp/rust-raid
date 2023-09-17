use std::cmp::Ordering::{Equal, Greater, Less};
use std::f32::consts::E;
/**
* Binary Search
*
* Binary Searching algorithm uses divide and conquer method to recursively find
* out elements of the item.
*
* It first compares middle most item of an array and recursively divides the
* array into sub-arrays until it finds out the exact element.
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

fn binary_search(array: &mut [i32], item: i32, start_index: usize) -> Result<usize, i8> {
    println!("{:?}", array);
    let mid_index = array.len() / 2;
    if array.len() == 1 && array[0] != item {
        // not existent
        return Err(1);
    }
    if item < array[0] || item > array[array.len() - 1] {
        // not in range
        return Err(-1);
    }
    return match array[mid_index].cmp(&item) {
        Less => binary_search(
            array[mid_index + 1..].as_mut(),
            item,
            start_index + mid_index + 1,
        ),
        Equal => Ok(start_index + mid_index),
        Greater => binary_search(array[..mid_index].as_mut(), item, start_index),
    };
}
fn main() {
    let mut arr_1 = [1, 4, 7, 8, 9, 10, 11, 12, 15, 20];
    arr_1.sort();
    let search_item = 12;
    let idx = binary_search(arr_1.as_mut(), search_item, 0).unwrap();
    println!("The item {} is at index: {}", search_item, idx);
}
#[cfg(test)]
mod tests {
    use crate::binary_search;

    #[test]
    fn search_ok() {
        assert_eq!(binary_search([1, 4, 2, 5, 7].as_mut(), 5, 0), Ok(3))
    }

    #[test]
    fn search_err() {
        assert_eq!(binary_search([1, 4, 2, 5, 7].as_mut(), 8, 0), Err(-1))
    }
}
