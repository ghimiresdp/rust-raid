//! To run the application, please run the following command
//! cargo run --bin doubly_linked_list
//! cargo run --test doubly_linked_list

// reference counter
use std::{borrow::BorrowMut, cell::RefCell, rc::Rc};

// since doubly linked list needs to have link for the previous and next nodes
// we need to use the reference counter
/**
 * Since doubly linked list needs to have link  to the same node from multiple
 * nodes. Example: in a linked list [A]<--->[B]<--->[c], both nodes [A] (next)
 * and [c] (prev) needs to reference the node [B].
 * As Rust does not allow ownership of the same resource, we need to work with
 * references, i.e. Reference Counter, which allows shared ownership.
 *
 * However, we can only have one mutable reference or multiple immutable refs.
 * In Doublylinked List we need to have both mutable and multiple references. To
 * fix this we need to use RefCell, which is an immutable reference but has
 * internal mutability which fixes our problem.
 */
type Link = Option<Rc<RefCell<Node>>>;

#[derive(Debug, Clone)]
struct Node {
    pub prev: Link,
    pub value: String,
    pub next: Link,
}

#[derive(Debug, Clone)]
struct DoublyLinkedList {
    pub head: Link,
    pub tail: Link,
}

impl Node {
    fn new(value: String) -> Self {
        Self {
            prev: None,
            next: None,
            value,
        }
    }
}

impl DoublyLinkedList {
    fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }
}

fn main() {
    // TODO
}
