/**
 * Consecutive groups problem is a problem in which we have to find the number
 * of groups needs to be created from a list of numbers. We have to create a new
 * group if there is a gap between 2 numbers.
 *
 * - example 1: [1,2,4,5,8,9,10]
 *   Here, this group should be grouped into [1,2], [4,5], [7,8,9]
 *   here, there is 3 skipped so we need to create a new group for 4,5 similarly,
 *   6 and 7 are skipped so we need to create another group for 8,9,10. Hence,
 *   the total number of groups to be created is 3.
 *
 * - example 2: [1,4,11,3,12,2,14,13]
 *   here we can see consecutive numbers [1,2,3,4] and [11,12,13,14], so the
 *   total number of groups to be created is 2.
 *
 */
use _lib::input; // core library for this repo

fn find_groups(mut list: Vec<usize>) -> usize {
    let mut sets = 1;
    list.sort();
    for (index, item) in list.iter().enumerate() {
        if index < list.len() - 1 && list[index + 1] - item != 1 {
            sets += 1;
        }
    }
    sets
}

fn main() {
    let value = input("enter numbers separated by space");
    println!("{}", value);
    let int_list: Vec<usize> = value
        .split(" ")
        .map(|v| v.trim().parse::<usize>().unwrap())
        .collect();
    println!("given: {:?}", int_list);
    println!("total groups: {}", find_groups(int_list));
}

#[cfg(test)]
mod test {
    use crate::find_groups;

    #[test]
    fn correct() {
        assert_eq!(find_groups(vec![1, 2, 4, 5, 6, 8, 9]), 3);
    }

    #[test]
    fn correct_with_unordered() {
        assert_eq!(find_groups(vec![1, 14, 2, 12, 3, 9, 15, 10, 8, 13]), 3)
    }
}
