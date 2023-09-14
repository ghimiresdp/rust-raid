use std::mem::swap;

/**
 * Selection Sort
 *
 * Selection sort is another sorting algorithm, which compares smallest of all
 * values and and put it in the first position. Similarly, after swapping value,
 * it will find out second most smallest value and swap it  with second item in
 * the array.
 *
 * Unlike bubble sort, the sorting will be done from the beginning.
 *
 * Example:
 *
 *  4 2 9 0 7 5 1   <- Swap 4 and 0
 *  ^     ^
 *  [0] 2 9 4 7 5 1 <- 0 is sorted
 *  [0] 2 9 4 7 5 1 <- Swap 2 and 1
 *      ^         ^
 * ...
 * Swap values until all values are sorted
**/
fn selection_sort(array: &mut [isize]) -> &mut [isize] {
    for i in 0..array.len(){
        let mut smallest = i;
        for j in  i..array.len(){
            if array[smallest] > array[j]{
                smallest = j;
            }
        }
        if smallest!=i{
            print!("Swapping {} and {} from {:?} -> ", array[smallest], array[i], array);
            array.swap(i, smallest);
            println!("{:?}", array)
        }
    }

    array
}

fn main() {
    let mut unsorted = [4, 2, 9, 0, 7, 5, 1];
    let sorted = selection_sort(unsorted.as_mut());

    println!("Sorted data: {:?}", sorted);
}

#[cfg(test)]
mod tests {
    use crate::selection_sort;

    #[test]
    fn sort_success() {
        assert_eq!(selection_sort([1, 2, 5, 4, 3, 6].as_mut()), [1, 2, 3, 4, 5, 6])
    }
}
