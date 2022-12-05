// Find the second largest item in the binary search tree
// https://www.interviewcake.com/question/javascript/second-largest-item-in-bst

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
    fn find_second_largest(&self) -> Option<i32> {
        let head = &self.head;

        if head.borrow().is_leaf() {
            return None;
        }

        let mut nodes = Vec::from([Rc::clone(head)]);

        loop {
            if let Some(current_node) = nodes.pop() {
                match &current_node.borrow().right {
                    // Case: current_node is parent of largest, and largest has no children,
                    // so current_node is 2nd largest
                    Some(right) => {
                        if right.borrow().is_leaf() {
                            return Some(current_node.borrow().value);
                        }
                        nodes.push(Rc::clone(right));
                    }
                    // Case: current_node is largest and has a left subtree
                    // 2nd largest is the largest in that subtree
                    None => {
                        if let Some(left) = &current_node.borrow().left {
                            return Some(BinaryTree::find_largest(Rc::clone(&left)));
                        }
                    }
                };
            };
        }
    }

    fn find_largest(root_node: RefNode<i32>) -> i32 {
        let mut nodes = Vec::from([root_node]);

        loop {
            if let Some(current_node) = nodes.pop() {
                match &current_node.borrow().right {
                    Some(right) => nodes.push(Rc::clone(right)),
                    None => return current_node.borrow().value,
                };
            }
        }
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
        let left = BTNode::new(value);
        self.left = Some(Rc::clone(&left));
        Rc::clone(&left)
    }

    fn insert_right(&mut self, value: T) -> RefNode<T> {
        let right = BTNode::new(value);
        self.right = Some(Rc::clone(&right));
        Rc::clone(&right)
    }

    fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_tree() {
        let mut tree_root = BinaryTree::new(50);
        let left_node = tree_root.insert_left(30);
        left_node.borrow_mut().insert_left(10);
        left_node.borrow_mut().insert_right(40);
        let right_node = tree_root.insert_right(70);
        right_node.borrow_mut().insert_left(60);
        right_node.borrow_mut().insert_right(80);

        assert_eq!(tree_root.find_second_largest(), Some(70));
    }

    #[test]
    fn largest_has_a_left_child() {
        let mut tree_root = BinaryTree::new(50);
        let left_node = tree_root.insert_left(30);
        left_node.borrow_mut().insert_left(10);
        left_node.borrow_mut().insert_right(40);
        let right_node = tree_root.insert_right(70);
        right_node.borrow_mut().insert_left(60);

        assert_eq!(tree_root.find_second_largest(), Some(60));
    }

    #[test]
    fn largest_has_a_left_subtree() {
        let mut tree_root = BinaryTree::new(50);
        let mut left_node = tree_root.insert_left(30);
        left_node.borrow_mut().insert_left(10);
        left_node.borrow_mut().insert_right(40);
        let right_node = tree_root.insert_right(70);
        left_node = right_node.borrow_mut().insert_left(60);
        left_node.borrow_mut().insert_right(65);
        left_node
            .borrow_mut()
            .insert_left(55)
            .borrow_mut()
            .insert_right(58);

        assert_eq!(tree_root.find_second_largest(), Some(65));
    }

    #[test]
    fn second_largest_is_root_node() {
        let mut tree_root = BinaryTree::new(50);
        let left_node = tree_root.insert_left(30);
        left_node.borrow_mut().insert_left(10);
        left_node.borrow_mut().insert_right(40);
        tree_root.insert_right(70);

        assert_eq!(tree_root.find_second_largest(), Some(50));
    }

    #[test]
    fn two_nodes_root_is_largest() {
        let mut tree_root = BinaryTree::new(50);
        tree_root.insert_left(30);

        assert_eq!(tree_root.find_second_largest(), Some(30));
    }

    #[test]
    fn second_largest_in_right_offshoot_left_subtree() {
        let mut tree_root = BinaryTree::new(50);
        let left_node = tree_root.insert_left(30);
        left_node.borrow_mut().insert_right(40);
        left_node.borrow_mut().insert_left(10);

        assert_eq!(tree_root.find_second_largest(), Some(40));
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

        assert_eq!(tree_root.find_second_largest(), Some(40));
    }

    #[test]
    fn ascending_linked_list() {
        let mut tree_root = BinaryTree::new(50);
        tree_root
            .insert_right(60)
            .borrow_mut()
            .insert_right(70)
            .borrow_mut()
            .insert_right(80);

        assert_eq!(tree_root.find_second_largest(), Some(70));
    }

    #[test]
    fn one_node_tree() {
        let tree_root = BinaryTree::new(50);

        assert_eq!(tree_root.find_second_largest(), None);
    }
}
