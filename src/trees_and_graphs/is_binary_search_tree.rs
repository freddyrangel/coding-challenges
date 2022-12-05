// Determine if the tree is a valid binary search tree
// https://www.interviewcake.com/question/javascript/bst-checker

#![allow(dead_code, unused_imports)]

use std::cell::RefCell;
use std::rc::Rc;

type RefNode<T> = Rc<RefCell<BTNode<T>>>;
type OptionNode<T> = Option<RefNode<T>>;

struct BinaryTree<T> {
    head: RefNode<T>,
}

struct BTNode<T> {
    value: T,
    left: OptionNode<T>,
    right: OptionNode<T>,
}

impl<T> BinaryTree<T> {
    fn new(value: T) -> Self {
        BinaryTree {
            head: BTNode::new(value),
        }
    }

    fn insert_left(&mut self, value: T) -> RefNode<T> {
        self.head.borrow_mut().insert_left(value)
    }

    fn insert_right(&mut self, value: T) -> RefNode<T> {
        self.head.borrow_mut().insert_right(value)
    }
}

impl BinaryTree<i32> {
    fn is_binary_search_tree(self) -> bool {
        let mut nodes_and_bounds = Vec::from([(self.head, i32::MIN, i32::MAX)]);

        while nodes_and_bounds.len() > 0 {
            if let Some((current_node, min, max)) = nodes_and_bounds.pop() {
                let current_value = current_node.borrow().value;

                if current_value >= max || min >= current_value {
                    return false;
                }

                if let Some(left) = &current_node.borrow().left {
                    nodes_and_bounds.push((Rc::clone(left), min, current_value));
                }

                if let Some(right) = &current_node.borrow().right {
                    nodes_and_bounds.push((Rc::clone(right), current_value, max));
                }
            }
        }

        true
    }
}

impl<T> BTNode<T> {
    fn new(value: T) -> RefNode<T> {
        Rc::new(RefCell::new(BTNode {
            value,
            left: None,
            right: None,
        }))
    }

    fn insert_left(&mut self, value: T) -> RefNode<T> {
        let node = BTNode::new(value);
        self.left = Some(Rc::clone(&node));
        Rc::clone(&node)
    }

    fn insert_right(&mut self, value: T) -> RefNode<T> {
        let node = BTNode::new(value);
        self.right = Some(Rc::clone(&node));
        Rc::clone(&node)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_full_tree() {
        let mut tree_root = BinaryTree::new(50);
        let left_node = tree_root.insert_left(30);
        left_node.borrow_mut().insert_left(10);
        left_node.borrow_mut().insert_right(40);
        let right_node = tree_root.insert_right(70);
        right_node.borrow_mut().insert_left(60);
        right_node.borrow_mut().insert_right(80);

        assert_eq!(tree_root.is_binary_search_tree(), true);
    }

    #[test]
    fn both_subtrees_valid() {
        let mut tree_root = BinaryTree::new(50);
        let left_node = tree_root.insert_left(30);
        left_node.borrow_mut().insert_left(20);
        left_node.borrow_mut().insert_right(60);
        let right_node = tree_root.insert_right(80);
        right_node.borrow_mut().insert_left(70);
        right_node.borrow_mut().insert_right(90);

        assert_eq!(tree_root.is_binary_search_tree(), false);
    }

    #[test]
    fn descending_linked_list() {
        let mut tree_root = BinaryTree::new(50);
        tree_root
            .insert_left(40)
            .borrow_mut()
            .insert_left(30)
            .borrow_mut()
            .insert_left(20)
            .borrow_mut()
            .insert_left(10);

        assert_eq!(tree_root.is_binary_search_tree(), true);
    }

    #[test]
    fn out_of_order_linked_list() {
        let mut tree_root = BinaryTree::new(50);
        tree_root
            .insert_right(70)
            .borrow_mut()
            .insert_right(60)
            .borrow_mut()
            .insert_right(80);

        assert_eq!(tree_root.is_binary_search_tree(), false);
    }

    #[test]
    fn one_node_tree() {
        let tree_root = BinaryTree::new(50);

        assert_eq!(tree_root.is_binary_search_tree(), true);
    }
}
