// Check if the linked list contains a cycle
// https://www.interviewcake.com/question/javascript/linked-list-cycles

#![allow(dead_code, unused_imports)]

use std::cell::RefCell;
use std::convert::From;
use std::rc::Rc;

type RefNode<T> = Rc<RefCell<LinkedListNode<T>>>;

struct LinkedList<T> {
    head: Option<RefNode<T>>,
}

#[derive(Debug, PartialEq, Eq)]
struct LinkedListNode<T> {
    value: T,
    next: Option<RefNode<T>>,
}

impl<T: Copy, const N: usize> From<[T; N]> for LinkedList<T> {
    fn from(values: [T; N]) -> Self {
        let mut nodes: Vec<RefNode<T>> = Vec::new();
        for (i, value) in values.iter().enumerate() {
            let node = LinkedListNode::new(value.clone());
            if i > 0 {
                let previous_node = Rc::clone(&nodes[i - 1]);
                previous_node.borrow_mut().next = Some(Rc::clone(&node));
            }
            nodes.push(node);
        }

        if values.len() > 0 {
            LinkedList {
                head: Some(Rc::clone(&nodes[0])),
            }
        } else {
            LinkedList { head: None }
        }
    }
}

impl<T: PartialEq> LinkedList<T> {
    fn contains_cycle(&self) -> bool {
        let mut nodes: Vec<RefNode<T>> = Vec::new();
        let mut node_option = self.head.clone();

        loop {
            match node_option.clone() {
                Some(node) => {
                    if nodes.contains(&node) {
                        return true;
                    } else {
                        nodes.push(Rc::clone(&node));
                        node_option = node.borrow().next.clone()
                    }
                }
                None => return false,
            }
        }
    }
}

impl<T> LinkedListNode<T> {
    fn new(value: T) -> RefNode<T> {
        Rc::new(RefCell::new(LinkedListNode { value, next: None }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linked_list_with_no_cycle() {
        let list = LinkedList::from([1, 2, 3, 4]);

        let mut node = Rc::clone(&list.head.as_ref().unwrap());
        assert_eq!(1, node.borrow().value);
        node = Rc::clone(&node).borrow().next.clone().unwrap();
        assert_eq!(2, node.borrow().value);
        node = Rc::clone(&node).borrow().next.clone().unwrap();
        assert_eq!(3, node.borrow().value);
        node = Rc::clone(&node).borrow().next.clone().unwrap();
        assert_eq!(4, node.borrow().value);
        assert_eq!(None, Rc::clone(&node).borrow().next);

        assert_eq!(list.contains_cycle(), false);
    }
    #[test]
    fn cycle_loops_to_beginning() {
        let list = LinkedList::from([1, 2, 3, 4]);
        let first_node = Rc::clone(&list.head.as_ref().unwrap());
        let mut last_node = Rc::clone(&first_node).borrow().next.clone().unwrap();
        last_node = Rc::clone(&last_node).borrow().next.clone().unwrap();
        last_node = Rc::clone(&last_node).borrow().next.clone().unwrap();

        Rc::clone(&last_node).borrow_mut().next = Some(first_node);

        assert_eq!(list.contains_cycle(), true);
    }

    #[test]
    fn cycle_loops_to_middle() {
        let list = LinkedList::from([1, 2, 3, 4, 5]);
        let mut second_node = Rc::clone(&list.head.as_ref().unwrap());
        second_node = Rc::clone(&second_node).borrow().next.clone().unwrap();

        let mut last_node = Rc::clone(&second_node).borrow().next.clone().unwrap();
        last_node = Rc::clone(&last_node).borrow().next.clone().unwrap();
        last_node = Rc::clone(&last_node).borrow().next.clone().unwrap();

        Rc::clone(&last_node).borrow_mut().next = Some(second_node);

        assert_eq!(list.contains_cycle(), true);
    }

    #[test]
    fn two_node_cycle_at_end() {
        let list = LinkedList::from([1, 2, 3, 4, 5]);
        let mut third_node = Rc::clone(&list.head.as_ref().unwrap());
        third_node = Rc::clone(&third_node).borrow().next.clone().unwrap();
        third_node = Rc::clone(&third_node).borrow().next.clone().unwrap();

        let last_node = Rc::clone(&third_node).borrow().next.clone().unwrap();

        Rc::clone(&last_node).borrow_mut().next = Some(third_node);

        assert_eq!(list.contains_cycle(), true);
    }

    #[test]
    fn empty_list() {
        let list: LinkedList<usize> = LinkedList::from([]);

        assert_eq!(list.contains_cycle(), false);
    }

    #[test]
    fn one_element_linked_list_no_cycle() {
        let list = LinkedList::from([1]);

        assert_eq!(list.contains_cycle(), false);
    }
}
