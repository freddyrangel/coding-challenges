// Determine if the tree is superbalanced
// https://www.interviewcake.com/question/javascript/balanced-binary-tree

#![allow(dead_code, unused_imports)]

use std::cell::RefCell;
use std::rc::Rc;

type RefNode<T> = Rc<RefCell<BTNode<T>>>;

struct BTNode<T> {
    value: T,
    left: Option<RefNode<T>>,
    right: Option<RefNode<T>>,
}

struct BinaryTree<T> {
    head: RefNode<T>,
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

    fn is_balanced(&self) -> bool {
        if self.head.borrow().is_leaf() {
            true
        } else {
            let levels = BinaryTree::collect_levels(Rc::clone(&self.head));

            let max_option = levels.iter().max();
            let min_option = levels.iter().min();

            match min_option.zip(max_option) {
                Some((min, max)) => max - min <= 1,
                None => false,
            }
        }
    }

    fn collect_levels(node: RefNode<T>) -> Vec<usize> {
        let mut levels: Vec<usize> = Vec::new();
        let mut nodes = Vec::from([(node, 0)]);

        while nodes.len() > 0 {
            if let Some((current_node, current_level)) = nodes.pop() {
                if current_node.borrow().is_leaf() {
                    if !levels.contains(&current_level) {
                        levels.push(current_level);
                    }
                } else {
                    if let Some(left) = &current_node.borrow().left {
                        nodes.push((Rc::clone(left), current_level + 1));
                    }
                    if let Some(right) = &current_node.borrow().right {
                        nodes.push((Rc::clone(right), current_level + 1));
                    }
                }
            }
        }

        levels
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

    fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_tree() {
        let mut tree_root = BinaryTree::new(5);
        let left_node = tree_root.insert_left(8);
        left_node.borrow_mut().insert_left(1);
        left_node.borrow_mut().insert_right(2);
        let right_node = tree_root.insert_right(6);
        right_node.borrow_mut().insert_left(3);
        right_node.borrow_mut().insert_right(4);

        assert_eq!(tree_root.is_balanced(), true);
    }

    #[test]
    fn both_leaves_at_the_same_depth() {
        let mut tree_root = BinaryTree::new(3);
        let left_node = tree_root.insert_left(4);
        left_node.borrow_mut().insert_left(1);
        let right_node = tree_root.insert_right(6);
        right_node.borrow_mut().insert_right(9);

        assert_eq!(tree_root.is_balanced(), true);
    }

    #[test]
    fn leaf_heights_differ_by_one() {
        let mut tree_root = BinaryTree::new(6);
        tree_root.insert_left(1);
        let right_node = tree_root.insert_right(0);
        right_node.borrow_mut().insert_right(7);

        assert_eq!(tree_root.is_balanced(), true);
    }

    #[test]
    fn leaf_heights_differ_by_two() {
        let mut tree_root = BinaryTree::new(6);
        tree_root.insert_left(1);
        let right_node = tree_root.insert_right(0);
        right_node
            .borrow_mut()
            .insert_right(7)
            .borrow_mut()
            .insert_right(8);

        assert_eq!(tree_root.is_balanced(), false);
    }

    #[test]
    fn three_leaves_total() {
        let mut tree_root = BinaryTree::new(1);
        tree_root.insert_left(5);
        let right_node = tree_root.insert_right(9);
        right_node.borrow_mut().insert_left(8);
        right_node.borrow_mut().insert_right(5);

        assert_eq!(tree_root.is_balanced(), true);
    }

    #[test]
    fn both_subtrees_superbalanced() {
        let mut tree_root = BinaryTree::new(1);
        tree_root.insert_left(5);
        let right_node = tree_root.insert_right(9);
        right_node
            .borrow_mut()
            .insert_left(8)
            .borrow_mut()
            .insert_left(7);
        right_node.borrow_mut().insert_right(5);

        assert_eq!(tree_root.is_balanced(), false);
    }

    #[test]
    fn both_subtrees_superbalanced_two() {
        let mut tree_root = BinaryTree::new(1);
        let left_node = tree_root.insert_left(2);
        left_node.borrow_mut().insert_left(3);
        left_node
            .borrow_mut()
            .insert_right(7)
            .borrow_mut()
            .insert_right(8);
        tree_root
            .insert_right(4)
            .borrow_mut()
            .insert_right(5)
            .borrow_mut()
            .insert_right(6)
            .borrow_mut()
            .insert_right(9);

        assert_eq!(tree_root.is_balanced(), false);
    }

    #[test]
    fn three_leaves_at_different_levels() {
        let mut tree_root = BinaryTree::new(1);
        let left_node = tree_root.insert_left(2);
        let left_left = left_node.borrow_mut().insert_left(3);
        left_node.borrow_mut().insert_right(4);
        left_left.borrow_mut().insert_left(5);
        left_left.borrow_mut().insert_right(6);
        tree_root
            .insert_right(7)
            .borrow_mut()
            .insert_right(8)
            .borrow_mut()
            .insert_right(9)
            .borrow_mut()
            .insert_right(10);

        assert_eq!(tree_root.is_balanced(), false);
    }

    #[test]
    fn only_one_node() {
        let tree_root = BinaryTree::new(1);

        assert_eq!(tree_root.is_balanced(), true);
    }

    #[test]
    fn linked_list_tree() {
        let mut tree_root = BinaryTree::new(1);
        tree_root
            .insert_right(2)
            .borrow_mut()
            .insert_right(3)
            .borrow_mut()
            .insert_right(4);

        assert_eq!(tree_root.is_balanced(), true);
    }
}
