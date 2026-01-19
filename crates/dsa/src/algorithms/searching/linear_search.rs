//! # Linear Searching Algorithm
//! To run/test, please run the following commands in your terminal
//!
//! ```sh
//! cargo run --bin linear-search
//! ```
//!
//! ```sh
//! cargo test --bin linear-search
//! ```
//!
//! Linear searching algorithm is a sequential searching algorithm, where we
//! traverse through each item by index and compare every element to the desired
//! element and return the index of the element.
//!
//! Example:
//!
//! Given array: [1, 5, 9, 2, 4], we want to search 9.
//!
//! step 1: check first item (index = 0), item = 1 , != 9
//! step 2: check second item (index = 1), item = 5 , != 9
//! step 3: check third item (index = 2), item = 9 , == 9 [return index : 2]
//!

use utils::parse_input;

fn linear_search(array: &mut [i32], item: i32) -> Result<usize, i8> {
    for (index, _item) in array.iter().enumerate() {
        if _item.eq(&item) {
            return Ok(index);
        }
    }
    return Err(-1);
}
fn main() {
    let mut array = [1, 5, 9, 2, 8, 0, 10, 13, 7, 4, 6];
    println!("Array: {:?}", array);
    if let Ok(search_item) = parse_input("Enter a number to search: ") {
        if let Ok(idx) = linear_search(array.as_mut(), search_item) {
            println!("The item {} is at index: {}", search_item, idx);
            return;
        }
        println!("The mumber '{search_item} does not exist in an array.'");
        return;
    };
    println!("Invalid number entered");
}

#[cfg(test)]
mod tests {
    use crate::linear_search;

    #[test]
    fn search_ok() {
        assert_eq!(linear_search([1, 4, 2, 5, 7].as_mut(), 5), Ok(3))
    }

    #[test]
    fn search_err() {
        assert_eq!(linear_search([1, 4, 2, 5, 7].as_mut(), 8), Err(-1))
    }
}
