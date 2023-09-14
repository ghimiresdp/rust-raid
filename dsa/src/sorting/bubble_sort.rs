/**
 * Bubble Sorting Algorithm
 *
 * Bubble sorting technique compares and swaps value one by one by traversing
 * through the list.
 *
 * If there are no swaps, the sorting is complete.
 *
 * example:
 *
 * step 1
 *  (4  2) 9  0  7  5  1     <- Swap 4 and 2
 *   2 (4  9) 0  7  5  1
 *   2  4 (9  0) 7  5  1     <- Swap 9 and 0
 *   2  4  0 (9  7) 5  1     <- Swap 9 and 7
 *   2  4  0  7 (9  5) 1     <- Swap 9 and 5
 *   2  4  0  7  5 (9  1)     <- Swap 9 and 1
 *   2  4  0  7  5  1 [9]     <- [9] is sorted
 *
 * Step 2 : 9 is sorted so sort remaining
 *
 *  (2  4) 0  7  5  1 [9]
 *   2 (4  0) 7  5  1 [9]   <- sort 4 and 0
 *  ...
 *   2  0  4  5  1 [7  9]     <- [7, 9] are sorted
 *
 *  do this process until every elements are sorted
 **/

fn bubble_sort(array: &mut [isize]) -> &mut [isize] {
    for i in 0..array.len() {
        println!("Step {}", i + 1);
        let mut swapped = false;
        for j in 0..array.len() - i - 1 {
            if array[j] > array[j + 1] {
                print!(
                    "Swapping values: {} and {} in {:?} -> ",
                    array[j],
                    array[j + 1],
                    array
                );
                array.swap(j, j + 1);
                swapped = true;
                println!("{:?}", array);
            }
        }
        if !swapped {
            break;
        }
    }
    array
}

fn main() {
    let mut unsorted = [4, 2, 9, 0, 7, 5, 1];
    let sorted = bubble_sort(unsorted.as_mut());

    println!("Sorted data: {:?}", sorted);
}

#[cfg(test)]
mod tests {
    use crate::bubble_sort;

    #[test]
    fn sort_success() {
        assert_eq!(bubble_sort([1, 2, 5, 4, 3, 6].as_mut()), [1, 2, 3, 4, 5, 6])
    }
}
