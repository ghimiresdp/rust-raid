/**
 *
 * Stacks
 * Stacks are data types that follows Last In First Out principle.
 * You can imaging stack as a stack of plates, or a packet of cookies.
 * The last item added in the stack is going to be picked to be used.
 *
 * The main purpose of this program is only to understand how stack works since
 * stacks are used everywhere in the low level. We will be using an example that
 * behaves as s stack, but in the memory, it might take either heap or stack.
 *
 * Rust's Vector data type already works as a stack, but we are wrapping it to
 * a custom structure named `Stack` to make the concept clearer.
 */

struct Stack<T> {
    content: Vec<T>, // container for items to store in the stack
}

impl<T> Stack<T> {
    fn new() -> Self {
        Self {
            content: Vec::new(),
        }
    }
    fn pop(&mut self) -> Option<T> {
        return self.content.pop();
    }
    fn push(&mut self, item: T) {
        self.content.push(item)
    }
}

/// To run / test, please run the following command in your terminal:
/// * cargo run --bin stack
/// * cargo test --bin stack
///
fn main() {
    let mut stack: Stack<i32> = Stack::new();
    stack.push(1);
    stack.push(2);
    println!("Last item: {}", stack.pop().unwrap()); // 2
    println!("Second Last item: {}", stack.pop().unwrap()); // 1
}

#[cfg(test)]
mod test {
    use crate::Stack;

    #[test]
    fn test_stack() {
        let mut stack: Stack<u32> = Stack::new();
        stack.push(0);
        stack.push(1);

        assert_eq!(stack.pop().unwrap(), 1); // last item
        assert_eq!(stack.pop().unwrap(), 0); // last item
    }

    #[test]
    fn test_empty_stack() {
        let mut stack: Stack<u32> = Stack::new();
        assert_eq!(stack.pop(), None) // no items to pop
    }
}
