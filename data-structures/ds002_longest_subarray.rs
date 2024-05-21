use std::collections::HashMap;

/**
 * -----------------------------------------------------------------------------
 * FIND THE LONGEST SUB-ARRAY WITH SUM K
 *
 * To execute, please run: cargo run --bin ds002
 * To run tests, please run: cargo test --bin ds002
 * -----------------------------------------------------------------------------
 *
 * Given an array of integers and an integer K, find the length of the longest
 * sub-array that sums up to K.
 *
 * EXAMPLE:
 * - input:
 *   - Array:[1, -1, 5, -2, 3]
 *   - K: 3
 * - output: 4 (the longest sub-array is [1, -1, 5, -2] and its length is 4)
 */

fn longest_subarray(array: Vec<i32>, k: i32) -> usize {
    let mut hm = HashMap::new();
    let mut sum = 0;
    let mut len = 0;

    for (idx, &num) in array.iter().enumerate() {
        sum += num;

        if sum == k {
            len = idx + 1;
        }
        if let Some(&hm_index) = hm.get(&(sum - k)) {
            len = len.max(idx - hm_index)
        }
        hm.entry(sum).or_insert(idx);
    }
    return len;
}

///
/// Visualization
/// iterations index(idx), values, hm_index: v
/// -+-----+-------+-------+-------+-------------------------+------------+
///  | idx |  num  |  sum  |  len  |  len if hm.get(sum - k) |  hm insert |
/// -+-----+-------+-------+-------+-------------------------+------------+
///  |  0  |   3   |   3   |   1   |     3 - 3  = 0 -> x     |   3 -> 0   |
/// -+-----+-------+-------+-------+-------------------------+------------+
///  |  1  |   1   |   4   |   "   |     4 - 3 = 1 -> x      |   4 -> 1   |
/// -+-----+-------+-------+-------+-------------------------+------------+
///  |  2  |  -1   |   3   |   3   |     3 - 3  = 0 -> x     |  NO INSERT |
/// -+-----+-------+-------+-------+-------------------------+------------+
///  |  3  |   5   |   8   |   "   |     8 - 3  = 5 -> x     |   8 -> 3   |
/// -+-----+-------+-------+-------+-------------------------+------------+
///  |  4  |  -2   |   6   |   "   |  v = hm.get(6 - 3) -> 0 |   6 -> 4   |
///  |     |       |       |       | idx - v = 4 - 0 = 4     |            |
///  |     |       |       |       | max(3, 4) = 4           |            |
/// -+-----+-------+-------+-------+-------------------------+------------+
fn main() {
    let array = vec![3, 1, -1, 5, -2];
    let k = 3;

    let sub_array_len = longest_subarray(array, k);
    println!("The largest sub array length is: {}", sub_array_len);
}

#[cfg(test)]
mod tests {
    use crate::longest_subarray;

    #[test]
    fn has_sub_array() {
        assert_eq!(longest_subarray(vec![3, 1, -1, 5, -2], 3), 4);
        assert_eq!(longest_subarray(vec![1, -1, 5, -2, 3], 0), 2);
        assert_eq!(longest_subarray(vec![1, -1, 5, -2, 3], 5), 4);
        assert_eq!(longest_subarray(vec![1, -1, 5, -2, 3], 4), 2);
    }

    #[test]
    fn no_sub_array() {
        assert_eq!(longest_subarray(vec![3, 1, -1, 5, -2], 7), 0);
    }
}
