/**
 * Insertion Sort
 *
 * Insertion Sort is the sorting algorithm, that virtually creates sub-arrays
 * of sorted and unsorted elements. Values from unsorted part are picked and
 * inserted at the correct position of the sorted element.
 *
 * Initially, first 2 elements of an array are compared and sorted if needed.
 * after that, second item is compared with the third element and sorted if
 * necessary (which may need sorting again within the sorted sub-array again)
 *
 * For example:
 *
 * 4 2 9 0 7 5 1
 *
 * [4 2] [9 0 7 5 1]    <- Sort 4 and 2 and add it to the sub array
 *  ^ ^
 * [2 4] [9 0 7 5 1]
 *    ^   ^
 * [2 4 9] [0 7 5 1]    <- Compare 4 and 9 as they do not need swapping, keep at is it
 *      ^   ^
 * [2 4 0 9] [7 5 1]    <- Compare and sort 9 and 0 (the sorted  sub-array again needs sorting)
 * [2 0 4 9] [7 5 1]    <- Compare and sort 4 and 0 (the sorted  sub-array again needs sorting)
 * [0 2 4 9] [7 5 1]    <- Compare and sort 2 and 0 (the sorted  sub-array again needs sorting)
 * ...
 * insert and sort until all items in an array is sorted
 **/
fn insertion_sort(array: &mut [isize]) -> &mut [isize] {
    for i in 1..array.len() {
        for pointer in (1..i + 1).rev() {
            if array[pointer] > array[pointer - 1] {
                break;
            }
            array.swap(pointer, pointer - 1);
        }
    }

    array
}

fn main() {
    let mut unsorted = [4, 2, 9, 0, 7, 5, 1];
    let sorted = insertion_sort(unsorted.as_mut());

    println!("Sorted data: {:?}", sorted);
}

#[cfg(test)]
mod tests {
    use crate::insertion_sort;

    #[test]
    fn sort_success() {
        assert_eq!(
            insertion_sort([8, 4, 2, 0, 3, 1, 9].as_mut()),
            [0, 1, 2, 3, 4, 8, 9]
        )
    }
}
