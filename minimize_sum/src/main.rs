use std::io;

/**
 * The task is to minimize the sum of integers when we pick `p` number of items
 * from the list of `n` items.
 * 
 * Here, If we want to pick 3 items from the list, of 6 items: [1, 2, 3, 4, 5, 6]
 * 
 * We can pick in the following order and find the sum of it:
 *  - 1st, 2nd, and max of remaining items:
 *       1, 2, max([3, 4, 5, 6]) = 1 + 2 + 6 = 9
 *  - 1st, max of remaining items except last, and the last item:
 *       1, max([2, 3, 4, 5]), 6 = 1 + 5 + 6 = 12
 *  - max of remaining items but (n-1)th and nth item, (n-1)th item, nth item:
 *       max([1, 2, 3, 4]), 5, 6 = 4 + 5 + 6 = 15
 *  out of above,  we have to find out the minimum of sum of above, which is 9
 * 
 **/
pub fn minimize_sum(list: Vec<i32>, pick: usize) -> i32 {
    let max_pick = list.len() - pick + 1;
    let mut outer = vec![0; pick];
    for a in 0..pick {
        let mut inner = vec![0; pick];
        for b in 0..pick {
            inner[b] =  if a == b {
                *list[b..max_pick+b].iter().max().unwrap()
            } else if a < b {
                list[b + max_pick-1]
            } else {
                list[b]
            }
            // if a == b {
            //     inner[b] = *list[b..max_pick+b].iter().max().unwrap();
            // } else if a < b {
            //     inner[b] = list[b + max_pick-1]
            // } else {
            //     inner[b] = list[b];
            // }
        }
        outer[a] = inner.iter().sum();
    }
    return *outer.iter().min().unwrap();
}

fn main() {
    
    println!("Enter spendings:");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let x = Vec::from_iter(buffer.trim().split(" ").map(|v| {v.parse::<i32>().unwrap()}));
    
    println!("Enter number of picks:");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let pick = buffer.trim().parse::<usize>().unwrap();

    println!("Minimum Spending is: {}", minimize_sum(x, pick))
}
