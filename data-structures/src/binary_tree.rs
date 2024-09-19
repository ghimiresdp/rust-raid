///! to run or test, please run the following commands:
/// ```shell
/// cargo run --bin binary_tree
/// cargo test --bin binary_tree
/// ```
///
use std::{cell::RefCell, fmt::Debug, rc::Rc};

type Node<T> = Rc<RefCell<BinaryTree<T>>>;

/// # Binary Tree
///
/// Binary Tree is a non-linear data structure that contains at most 2 children.
/// Specifically, left node and right node.
///
/// In Rustlang, we can create a binary tree by adding optional values to the
/// left and right along with the data of the node.
///
/// A perfect binary tree is a binary tree in which all parent nodes contains
/// both `left` and `right` nodes.
///
/// An example of a perfect binary tree is shown below:
///
/// ```
///            (7)
///         /      \
///      (5)        (6)
///    /    \     /    \
/// (1)     (2) (3)    (4)
///
/// ```
/// We Use Option data type so that we can choose either `None` or `Some` values
/// for their children.
/// We also use `Reference Counter` so that it can be accessed multiple times
/// while adding new nodes.
#[derive(Debug, Clone)]
struct BinaryTree<T: Debug> {
    data: T,
    left: Option<Node<T>>,
    right: Option<Node<T>>,
}

impl<T: Debug> BinaryTree<T> {
    fn new(data: T) -> Self {
        Self {
            data,
            left: None,
            right: None,
        }
    }
    fn new_node(data: T) -> Node<T> {
        Rc::new(RefCell::new(BinaryTree::new(data)))
    }
    fn set_left(&mut self, node: Node<T>) {
        self.left = Some(node);
    }
    fn set_right(&mut self, node: Node<T>) {
        self.right = Some(node);
    }

    /// Get the total length of the tree.
    /// * If the tree is empty, it's length is always 1 since it always has data.
    /// * If the tree has children, it's length is equal to 1 + length of its both children.
    fn length(&self) -> usize {
        let mut len = 1; // if it has no children, then the length is  always 1

        if let Some(left) = &self.left {
            len += left.borrow().length()
        }
        if let Some(right) = &self.right {
            len += right.borrow().length()
        }
        len
    }
}

fn create_binary_tree() -> BinaryTree<i32> {
    let mut btree = BinaryTree::new(7);
    btree.set_left(BinaryTree::new_node(5));
    btree.set_right(BinaryTree::new_node(6));

    if let Some(left) = &btree.left {
        left.try_borrow_mut()
            .unwrap()
            .set_left(BinaryTree::new_node(1));
        left.try_borrow_mut()
            .unwrap()
            .set_right(BinaryTree::new_node(2));
    }

    if let Some(right) = &btree.right {
        right
            .try_borrow_mut()
            .unwrap()
            .set_left(BinaryTree::new_node(3));
        right
            .try_borrow_mut()
            .unwrap()
            .set_right(BinaryTree::new_node(4));
    }
    btree
}

fn main() {
    let btree = create_binary_tree();
    println!("Root Node Data: {:?}", btree.data);
    println!("Binary Tree Length: {:?}", btree.length());
    println!("Binary Tree: {:?}", btree);
}

#[cfg(test)]
mod tests {
    use crate::create_binary_tree;

    #[test]
    fn binary_tree() {
        let tree = create_binary_tree();
        assert_eq!(tree.left.unwrap().borrow().data, 5);
        assert_eq!(tree.right.unwrap().borrow().data, 6);
    }

    #[test]
    fn binary_tree_length() {
        let tree = create_binary_tree();
        assert_eq!(tree.length(), 7);
    }
}
