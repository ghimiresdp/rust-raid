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
    table[0] = 0;
    for num in 1..sum + 1 {
        for coin in 1..coins.len() {
            if coins[coin] <= num {
                // todo
                let sub_result = table[num - coins[coin]];
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

    // // discard change if the sum is already smaller than the smallest coin
    // if sum < coins.last().unwrap().to_owned() {
    //     return None;
    // }

    // let mut remaining_amount = sum;
    // let mut changes: Vec<usize> = Vec::new();

    // for idx in 0..coins.len() {
    //     let coin = coins[idx];
    //     if remaining_amount == 0 {
    //         break;
    //     }
    //     if coin > remaining_amount {
    //         continue;
    //     };

    //     let coins = remaining_amount / coin;
    //     remaining_amount = remaining_amount % coin;
    //     let mut v = vec![coin; coins];
    //     changes.append(v.as_mut());
    // }
    // if remaining_amount > 0 {
    //     return None;
    // }
    // return Some(changes.len());
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
