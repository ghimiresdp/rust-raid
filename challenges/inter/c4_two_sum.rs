/// Two Sum Problem
/// https://leetcode.com/problems/two-sum/
/// Given a vector of integers and an integer target, return indices of the num
///

pub fn main() {
    let numbers = two_sum(vec![2, 7, 11, 15], 9);
    println!("{:?}", numbers);
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for num_1 in 0..nums.len() {
        for num_2 in num_1 + 1..nums.len() {
            if nums[num_1] + nums[num_2] == target {
                return vec![num_1 as i32, num_2 as i32];
            }
        }
    }
    vec![-1, -1]
}

#[cfg(test)]
mod tests {
    use super::two_sum;
    fn test_two_sum() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    }
}
