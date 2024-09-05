/**
 * m5: overflow-aware integer reverse
 *
 * The solution reverses an integer that is overflow-aware.
 *
 * If the integer overflows while reversing its digits, it will return 0
 *
 *
 * To run or test codes, run the following:
 * =============================================================================
 *
 * cargo run --bin m5
 * cargo test --bin m5
 *
 * =============================================================================
 */
struct Solution;

impl Solution {
    pub fn reverse(mut x: i32) -> i32 {
        let mut result: i32 = 0;
        loop {
            result = match result.checked_mul(10).and_then(|n| n.checked_add(x % 10)) {
                Some(res) => res,
                None => {
                    result = 0;
                    break;
                }
            };
            x /= 10;
            if x == 0 {
                break;
            };
        }
        result
    }
}

fn main() {
    println!(
        "reverse of 2147483647 is: {}",
        Solution::reverse(2147483647)
    );
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_reverse() {
        assert_eq!(Solution::reverse(1024), 4201);
    }

    #[test]
    fn test_reverse_overflow() {
        assert_eq!(Solution::reverse(2147483647), 0);
    }
}
