/*
 * AVL TREE
 * https://btholt.github.io/complete-intro-to-computer-science/avl-tree
 * Create an AVL Tree
 */

use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

type RefAVLNode = Rc<RefCell<Box<AVLNode>>>;

#[derive(Debug)]
struct AVLTree {
    head: Option<RefAVLNode>,
}

#[derive(Debug)]
struct AVLNode {
    value: usize,
    height: usize,
    parent: Option<RefAVLNode>,
    left: Option<RefAVLNode>,
    right: Option<RefAVLNode>,
}

impl AVLTree {
    fn new() -> Self {
        AVLTree { head: None }
    }

    fn add(&mut self, value: usize) {
        if let Some(head) = &self.head {
            head.clone().borrow_mut().add(value);
        } else {
            self.head = Some(AVLNode::new(value));
        }
    }
}

impl AVLNode {
    fn new(value: usize) -> RefAVLNode {
        Rc::new(RefCell::new(Box::new(AVLNode {
            value,
            height: 1,
            parent: None,
            left: None,
            right: None,
        })))
    }

    fn add(&mut self, value: usize) {
        if value < self.value {
            // Go left

            if let Some(left) = &self.left {
                left.clone().borrow_mut().add(value);
            } else {
                self.left = Some(AVLNode::new(value));
            }

            if let Some(left) = &self.left {
                if self.right.is_none()
                    || self.right.clone().unwrap().borrow().height < left.borrow().height
                {
                    self.height = left.borrow().height + 1;
                }
            }
        } else {
            // Go right

            if let Some(right) = &self.right {
                right.clone().borrow_mut().add(value);
            } else {
                self.right = Some(AVLNode::new(value));
            }

            if let Some(right) = &self.right {
                if self.left.is_none()
                    || right.borrow().height > self.left.clone().unwrap().borrow().height
                {
                    self.height = right.borrow().height + 1;
                }
            }
        }
        self.balance();
    }

    fn balance(&mut self) {
        let right_height = if let Some(right) = &self.right {
            right.clone().borrow().height
        } else {
            0
        };

        let left_height = if let Some(left) = &self.left {
            left.clone().borrow().height
        } else {
            0
        };

        if left_height > right_height + 1 {
            let left_right_height = if let Some(right) = &self
                .left
                .clone()
                .and_then(|left| left.clone().borrow().right.clone())
            {
                right.borrow().height
            } else {
                0
            };

            let left_left_height = if let Some(left) = &self
                .left
                .clone()
                .and_then(|left| left.clone().borrow().left.clone())
            {
                left.clone().borrow().height
            } else {
                0
            };

            if left_right_height > left_left_height {
                self.left.clone().unwrap().clone().borrow_mut().rotate_rr();
            }

            self.rotate_ll();
        } else if right_height > left_height + 1 {
            let right_right_height = if let Some(right) = &self
                .right
                .clone()
                .and_then(|right| right.clone().borrow().right.clone())
            {
                right.borrow().height
            } else {
                0
            };
            let right_left_height = if let Some(left) = &self
                .right
                .clone()
                .and_then(|right| right.clone().borrow().left.clone())
            {
                left.borrow().height
            } else {
                0
            };

            if right_left_height > right_right_height {
                self.right.clone().unwrap().clone().borrow_mut().rotate_ll();
            }

            self.rotate_rr();
        }
    }

    fn rotate_rr(&mut self) {
        let value_before = self.value;
        let left_before = self.left.clone();
        self.value = self.right.clone().unwrap().borrow().value;
        self.left = self.right.clone();
        self.right = self.right.clone().unwrap().borrow().right.clone();
        let temp = self.left.clone().unwrap();
        let mut left_ref = temp.borrow_mut();
        left_ref.right = left_ref.left.clone();
        left_ref.left = left_before;
        left_ref.value = value_before;
        left_ref.update_in_new_location();
        self.update_in_new_location();
    }

    fn rotate_ll(&mut self) {
        let value_before = self.value;
        let right_before = self.right.clone();
        self.value = self.left.clone().unwrap().borrow().value;
        self.right = self.left.clone();
        self.left = self.left.clone().unwrap().borrow().left.clone();
        let temp = self.right.clone().unwrap();
        let mut right_ref = temp.borrow_mut();
        right_ref.left = right_ref.right.clone();
        right_ref.right = right_before;
        right_ref.value = value_before;
        right_ref.update_in_new_location();
        self.update_in_new_location();
    }

    fn update_in_new_location(&mut self) {
        if self.right.is_none() && self.left.is_none() {
            self.height = 1;
        } else if self.right.is_none()
            || (self.left.is_some()
                && self.right.clone().unwrap().borrow().height
                    < self.left.clone().unwrap().borrow().height)
        {
            self.height = self.left.clone().unwrap().borrow().height + 1;
        } else {
            self.height = self.right.clone().unwrap().borrow().height + 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn it_creates_avl_tree() {
        let nums = vec![3, 7, 4, 6, 5, 1, 10, 2, 9, 8];
        let mut tree = AVLTree::new();

        nums.iter().for_each(|num| tree.add(*num));
        println!("TREE: {:?}", tree);

        let head = tree.head.unwrap();

        assert_eq!(head.borrow().value, 4);

        let node = head.clone().borrow().left.clone().unwrap();

        assert_eq!(node.borrow().value, 2);

        //assert_eq!(head.left.as_ref().unwrap().left.as_ref().unwrap().value, 1);
        //assert_eq!(
        //head.left
        //.as_ref()
        //.unwrap()
        //.left
        //.as_ref()
        //.unwrap()
        //.left
        //.as_ref()
        //.is_none(),
        //true
        //);
        //assert_eq!(
        //head.left
        //.as_ref()
        //.unwrap()
        //.left
        //.as_ref()
        //.unwrap()
        //.right
        //.as_ref()
        //.is_none(),
        //true
        //);

        //assert_eq!(head.left.as_ref().unwrap().right.as_ref().unwrap().value, 3);
        //assert_eq!(
        //head.left
        //.as_ref()
        //.unwrap()
        //.right
        //.as_ref()
        //.unwrap()
        //.left
        //.as_ref()
        //.is_none(),
        //true
        //);
        //assert_eq!(
        //head.left
        //.as_ref()
        //.unwrap()
        //.right
        //.as_ref()
        //.unwrap()
        //.right
        //.as_ref()
        //.is_none(),
        //true
        //);

        //assert_eq!(head.right.as_ref().unwrap().value, 7);

        //assert_eq!(head.right.as_ref().unwrap().left.as_ref().unwrap().value, 6);
        //assert_eq!(
        //head.right
        //.as_ref()
        //.unwrap()
        //.left
        //.as_ref()
        //.unwrap()
        //.right
        //.as_ref()
        //.is_none(),
        //true
        //);

        //assert_eq!(
        //head.right
        //.as_ref()
        //.unwrap()
        //.left
        //.as_ref()
        //.unwrap()
        //.left
        //.as_ref()
        //.unwrap()
        //.value,
        //5
        //);
        //assert_eq!(
        //head.right
        //.as_ref()
        //.unwrap()
        //.left
        //.as_ref()
        //.unwrap()
        //.left
        //.as_ref()
        //.unwrap()
        //.left
        //.as_ref()
        //.is_none(),
        //true
        //);
        //assert_eq!(
        //head.right
        //.as_ref()
        //.unwrap()
        //.left
        //.as_ref()
        //.unwrap()
        //.left
        //.as_ref()
        //.unwrap()
        //.right
        //.as_ref()
        //.is_none(),
        //true
        //);

        //assert_eq!(
        //head.right.as_ref().unwrap().right.as_ref().unwrap().value,
        //9
        //);

        //assert_eq!(
        //head.right
        //.as_ref()
        //.unwrap()
        //.right
        //.as_ref()
        //.unwrap()
        //.left
        //.as_ref()
        //.unwrap()
        //.value,
        //8
        //);
        //assert_eq!(
        //head.right
        //.as_ref()
        //.unwrap()
        //.right
        //.as_ref()
        //.unwrap()
        //.left
        //.as_ref()
        //.unwrap()
        //.left
        //.as_ref()
        //.is_none(),
        //true
        //);
        //assert_eq!(
        //head.right
        //.as_ref()
        //.unwrap()
        //.right
        //.as_ref()
        //.unwrap()
        //.left
        //.as_ref()
        //.unwrap()
        //.right
        //.as_ref()
        //.is_none(),
        //true
        //);

        //assert_eq!(
        //head.right
        //.as_ref()
        //.unwrap()
        //.right
        //.as_ref()
        //.unwrap()
        //.right
        //.as_ref()
        //.unwrap()
        //.value,
        //10
        //);
        //assert_eq!(
        //head.right
        //.as_ref()
        //.unwrap()
        //.right
        //.as_ref()
        //.unwrap()
        //.right
        //.as_ref()
        //.unwrap()
        //.left
        //.as_ref()
        //.is_none(),
        //true
        //);
        //assert_eq!(
        //head.right
        //.as_ref()
        //.unwrap()
        //.right
        //.as_ref()
        //.unwrap()
        //.right
        //.as_ref()
        //.unwrap()
        //.right
        //.as_ref()
        //.is_none(),
        //true
        //);
    }
}
