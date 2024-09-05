/**
 * Median of two sorted vectors
 *
 * Median of two sorted vectors challenge
 *
 * To run the following, run:
 * =============================================================================
 *
 * cargo run --bin m6
 * cargo test --bin m6
 *
 * =============================================================================
 */

struct Solution;

impl Solution {
    fn find_median_sorted_arrays(mut nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        nums1.extend(nums2);
        nums1.sort();

        let high = nums1.len() / 2;
        match nums1.len() % 2 {
            1 => nums1[high] as f64,
            _ => (nums1[high] as f64 + nums1[high - 1] as f64) / 2f64,
        }
    }
}

fn main() {
    let (v1, v2) = (vec![1, 3], vec![2]);
    println!(
        "the median value of the following vectors {:?}, {:?} is: {}",
        v1.clone(),
        v2.clone(),
        Solution::find_median_sorted_arrays(v1, v2)
    );
    let (v1, v2) = (vec![1, 2], vec![3, 4]);
    println!(
        "the median value of the following vectors {:?}, {:?} is: {}",
        v1.clone(),
        v2.clone(),
        Solution::find_median_sorted_arrays(v1, v2)
    );
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_median_odd_elements() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
            2f64
        )
    }
    #[test]
    fn test_median_even_elements() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
            2.5
        )
    }
}
