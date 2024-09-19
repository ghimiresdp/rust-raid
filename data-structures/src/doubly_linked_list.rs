//! To run the application, please run the following command
//! cargo run --bin doubly_linked_list
//! cargo run --test doubly_linked_list

// reference counter
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

// since doubly linked list needs to have link for the previous and next nodes
// we need to use the reference counter
/// Since doubly linked list needs to have link  to the same node from multiple
/// nodes. Example: in a linked list [A]<--->[B]<--->[c], both nodes [A] (next)
/// and [c] (prev) needs to reference the node [B].
/// As Rust does not allow ownership of the same resource, we need to work with
/// references, i.e. Reference Counter, which allows shared ownership.
///
/// However, we can only have one mutable reference or multiple immutable refs.
/// In Doubly linked List we need to have both mutable and multiple references.
/// To fix this we need to use RefCell, which is an immutable reference but has
/// internal mutability which fixes our problem.
///
/// Also the `Weak` Reference counter is used for the `prev` option since we can
/// use the non-owning reference to the managed allocation. This will return us
/// the Option<Rc<Node>> instead of Rc<Node>. This is specially useful when we
/// pop out the value in the list.

struct Node {
    pub value: u32,
    pub prev: Option<Weak<RefCell<Node>>>,
    pub next: Option<Rc<RefCell<Node>>>,
}

type NodePointer = Rc<RefCell<Node>>;

struct DoublyLinkedList {
    pub head: Option<NodePointer>,
    pub tail: Option<NodePointer>,
}

impl Node {
    fn new(value: u32) -> Self {
        Self {
            prev: None,
            next: None,
            value,
        }
    }
}
impl From<Node> for Option<NodePointer> {
    fn from(node: Node) -> Self {
        Some(Rc::new(RefCell::new(node)))
    }
}

impl DoublyLinkedList {
    fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }
    fn push_back(&mut self, value: u32) {
        let mut node = Node::new(value);
        match &mut self.tail.take() {
            None => {
                self.head = node.into();
                self.tail = self.head.clone();
            }
            Some(current_tail) => {
                node.prev = Some(Rc::downgrade(&current_tail));
                self.tail = node.into();
                // current_tail.borrow_mut().next = self.tail.clone();
                current_tail.try_borrow_mut().unwrap().next = self.tail.clone();
            }
        }
    }
}

fn main() {
    let mut dll = DoublyLinkedList::new();
    dll.push_back(0); // DLL < 0 >
    dll.push_back(1); // DLL < 0, 1 >
    dll.push_back(2); // DLL < 0, 1, 2 >
    if let Some(value) = dll.head {
        println!("{}", value.try_borrow_mut().unwrap().value);
    }
}

#[cfg(test)]
mod tests {
    use crate::DoublyLinkedList;

    #[test]
    fn creates_list() {
        let mut dll = DoublyLinkedList::new();
        dll.push_back(1);
        dll.push_back(2);
        dll.push_back(3);
        dll.push_back(4);

        assert_eq!(dll.head.clone().unwrap().try_borrow().unwrap().value, 1);
        assert_eq!(dll.tail.clone().unwrap().try_borrow().unwrap().value, 4);

        let next = dll.head.clone().unwrap().try_borrow().unwrap().next.clone();
        // get second node
        assert_eq!(next.unwrap().try_borrow().unwrap().value, 2);
    }
}
