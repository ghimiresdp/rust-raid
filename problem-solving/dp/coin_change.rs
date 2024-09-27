/// To run/test the solution, please run the following commands:
/// * cargo run --bin coin_change
/// * cargo test --bin coin_change
use std::usize;

/// The coin change problem gives us a solution to the minimum number of coins
/// required to change the given amount with the given coin options, considering
/// that any number of coins can be used from the given coin options.
pub fn coin_change(coins: &mut Vec<usize>, sum: usize) -> i64 {
    if sum == 0 {
        return 0;
    }
    // check if no coins are provided
    if coins.len() == 0 {
        return -1;
    }

    // initialize an array of maximum number to store the minimum number of
    // coins required to change the given amount
    let mut table = vec![usize::MAX; sum + 1];

    // the first item of the table is always 0 since 0 can be changed with 0 number of coins.
    table[0] = 0;

    // iterate from 1 up to the total sum
    for num in 1..sum + 1 {
        // iterate on each coin from the list of coins
        for coin in 1..coins.len() {
            // discard the coin if the number (intermediate sum) is smaller than the coin itself
            if coins[coin] <= num {
                // find the number of coins required to change that specific amount.
                // for this, we can check  previous calculation and add the number of coins by 1
                // if it can be changed.
                // example: if we were about to change $50, and we already know that
                // we have 5 changes for $45 and we have $5 coin for change, we do not need to
                // find it from the start.
                let sub_result = table[num - coins[coin]];

                // check if the sub result is smallest when iterated over all the available coins.
                // example: if sub_result when using coin 8 is smaller than using the coin 5,
                // we should choose the sub-result with coin 8 since it will give us smallest
                // number of coins to change the specified amount.
                if sub_result != usize::MAX && sub_result + 1 < table[num] {
                    table[num] = sub_result + 1;
                }
            }
        }
    }
    if table[sum] == usize::MAX {
        return -1;
    }
    return table[sum] as i64;
}

fn main() {
    let mut coins: Vec<usize> = vec![1, 2, 5, 10];
    let amount = 49;
    let changes = coin_change(coins.as_mut(), amount);
    match changes {
        -1 => {
            println!("{} can not be changed with {:?}", amount, coins);
        }
        v => {
            println!(
                "{} an be changed into: {:?} with coins of {:?}",
                amount, v, coins
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::coin_change;

    #[test]
    fn test0() {
        assert_eq!(coin_change(vec![1].as_mut(), 0), 0);
    }

    #[test]
    fn test1() {
        assert_eq!(coin_change(vec![1, 2, 5, 10].as_mut(), 49), 7);
    }

    #[test]
    fn test2() {
        assert_eq!(coin_change(vec![2, 7, 10].as_mut(), 21), 3);
    }
    #[test]
    fn test3() {
        assert_eq!(coin_change(vec![9, 6, 5, 1].as_mut(), 11), 2);
    }
}
