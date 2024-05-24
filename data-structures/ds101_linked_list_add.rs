/**
 * -----------------------------------------------------------------------------
 * ADD TWO LINKED LISTS
 *
 * To execute, please run: cargo run --bin ds101
 * To run tests, please run: cargo test --bin ds101
 * -----------------------------------------------------------------------------
 *
 * Given linked list that contains i32 number as a digit of each place, perform
 * addition of two linked lists an return a new linked list.
 *
 * Example A linked list [4, 2, 0, 1] will represent a number 1024
 *
 * Additionally, implement a linked list that could perform addition operation
 *
 * Example:
 *  512 + 512 = 1024
 *  - Input linked list: [2,1,5], [2,1,5]
 *  - Output linked list: [4, 2, 0, 1]
 *
 */

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    fn from_str(value: &str) -> Self {
        if let Some(first_char) = value.chars().next() {
            return Self {
                val: first_char.to_string().parse::<i32>().unwrap(),
                next: match &value[1..] {
                    "" => None,
                    v => Some(Box::from(Self::from_str(&v))),
                },
            };
        }
        panic!("the string of length greater than 0 is expected");
    }

    fn to_string(&self) -> String {
        format!(
            "{}{}",
            self.val,
            match self.next.clone() {
                Some(node) => node.to_string(),
                None => String::new(),
            }
        )
    }
}

//---------------------------------[ solution ]---------------------------------

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut l1 = l1;
    let mut l2 = l2;
    let mut carry = 0;
    let mut result_str = String::new();

    loop {
        let sum = match (l1.clone(), l2.clone(), carry) {
            (None, None, 0) => break,
            (None, None, carry) => carry,
            (None, Some(b), carry) => {
                l2 = b.next;
                b.val + carry
            }
            (Some(a), None, carry) => {
                l1 = a.next;
                a.val + carry
            }
            (Some(a), Some(b), carry) => {
                l1 = a.next;
                l2 = b.next;
                a.val + b.val + carry
            }
        };
        let val = sum % 10;
        carry = sum / 10;
        result_str = format!("{}{}", result_str, val);
    }
    Some(Box::new(ListNode::from_str(&result_str)))
}

fn main() {
    let l1 = Some(Box::new(ListNode::from_str("215")));
    let l2 = Some(Box::new(ListNode::from_str("215")));
    let result = add_two_numbers(l1, l2);
    println!("result: {}", result.unwrap().to_string());
}

#[cfg(test)]
mod tests {
    use crate::{add_two_numbers, ListNode};

    #[test]
    fn test_sum() {
        assert_eq!(
            add_two_numbers(
                Some(Box::new(ListNode::from_str("9"))),
                Some(Box::new(ListNode::from_str("199999999"))),
            )
            .unwrap()
            .to_string(),
            "0000000001"
        )
    }
}
