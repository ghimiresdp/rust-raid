/// Two Sum Problem [from leetcode]
/// https://leetcode.com/problems/two-sum/
use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut maps: HashMap<i32, i32> = HashMap::new();
    for num_1 in 0..nums.len() {
        let required = target - nums[num_1];
        if let Some(value) = maps.get(&required) {
            return vec![*value, num_1 as i32];
        }
        maps.insert(nums[num_1], num_1 as i32);
    }
    vec![-1, -1]
}
pub fn main() {
    let numbers = two_sum(vec![2, 7, 11, 15], 9);
    println!("{:?}", numbers);
}

#[cfg(test)]
mod tests {
    use super::two_sum;

    #[test]
    fn test_two_sum() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(two_sum(vec![3, 2, 4], 9), vec![-1, -1]);
    }
}
