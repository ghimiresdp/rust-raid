/**
 * Quick sort is an efficient algorithm that follows divide-and-conquer
 * approach
 *
 * Quick sorting process consists of the following steps:
 *
 * 1. Pivot selection   : Choose pivot point/element from the array
 * 2. Partitioning      : Rearrange the array
 * 3. Recursion         : Split and select pivot point until sub-array contains single element
 * 4. Combination       : Rearrange and combine all single items
 **/

fn partition(array: &mut [i32]) -> usize {
    let mut tracker = 0;

    // we initially assume the last index as the pivot index
    let pivot = array.len() - 1;

    // iterate through the array/slice of array and move elements smaller than
    // the pivot to the left
    for index in 0..pivot {
        if array[index] <= array[pivot] {
            array.swap(index, tracker);
            tracker += 1;
        }
    }

    // move the pivot to the correct sorted position
    array.swap(tracker, pivot);
    tracker
}
fn quick_sort(array: &mut [i32]) -> &mut [i32] {
    if array.len() > 1 {
        let pivot = partition(array);
        quick_sort(&mut array[..pivot]);
        quick_sort(&mut array[pivot + 1..]);
    }
    array
}
fn main() {
    let mut unsorted = [4, 2, 9, 0, 7, 5, 1];
    let sorted = quick_sort(unsorted.as_mut());
    println!("Sorted data: {:?}", sorted);
}

#[cfg(test)]
mod tests {
    use crate::quick_sort;

    #[test]
    fn sort_success() {
        assert_eq!(quick_sort([1, 2, 5, 4, 3, 6].as_mut()), [1, 2, 3, 4, 5, 6])
    }
}
