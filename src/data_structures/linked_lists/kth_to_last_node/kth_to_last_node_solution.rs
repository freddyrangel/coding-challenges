/*
 * KTH TO LAST NODE
 * https://www.interviewcake.com/question/javascript/kth-to-last-node-in-singly-linked-list
 * Return the kth to last node in the linked list
 */

use std::cell::RefCell;
use std::convert::From;
use std::fmt::Debug;
use std::rc::Rc;

type RefNode<T> = Rc<RefCell<LinkedListNode<T>>>;
type RefNodeList<T> = Vec<RefNode<T>>;

struct LinkedList<T> {
    head: Option<RefNode<T>>,
}

struct LinkedListNode<T> {
    value: T,
    next: Option<RefNode<T>>,
}

impl<T: Copy, const N: usize> From<[T; N]> for LinkedList<T> {
    fn from(values: [T; N]) -> Self {
        let mut nodes: RefNodeList<T> = Vec::new();
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

impl<T> LinkedList<T> {
    fn kth_to_last_node(&self, kth: usize) -> Result<RefNode<T>, &str> {
        let reversed_nodes: RefNodeList<T> = self.nodes().into_iter().rev().collect();

        for (i, node) in reversed_nodes.iter().enumerate() {
            if i == kth {
                return Ok(Rc::clone(node));
            }
        }

        Err("Doesn't exit")
    }

    fn nodes(&self) -> RefNodeList<T> {
        let mut nodes: RefNodeList<T> = Vec::new();
        let mut current_node_option = self.head.clone();

        loop {
            match current_node_option {
                Some(current_node) => {
                    nodes.push(Rc::clone(&current_node));
                    current_node_option = Rc::clone(&current_node).borrow().next.clone();
                }
                None => {
                    break;
                }
            }
        }

        nodes
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
    fn first_to_last_node() {
        let list = LinkedList::from([1, 2, 3, 4]);
        let actual = list.kth_to_last_node(1).unwrap().borrow().value;
        let expected = 3;
        assert_eq!(actual, expected);
    }

    #[test]
    fn second_to_last_node() {
        let list = LinkedList::from([1, 2, 3, 4]);
        let actual = list.kth_to_last_node(2).unwrap().borrow().value;
        let expected = 2;
        assert_eq!(actual, expected);
    }

    #[test]
    fn first_node() {
        let list = LinkedList::from([1, 2, 3, 4]);
        let actual = list.kth_to_last_node(3).unwrap().borrow().value;
        let expected = 1;
        assert_eq!(actual, expected);
    }

    #[test]
    fn k_greater_than_linked_list_length() {
        let list = LinkedList::from([1, 2, 3, 4]);
        let actual = list.kth_to_last_node(4);

        assert_eq!(actual.is_err(), true);
    }

    #[test]
    fn k_is_zero() {
        let list = LinkedList::from([1, 2, 3, 4]);
        let actual = list.kth_to_last_node(0).unwrap().borrow().value;
        let expected = 4;
        assert_eq!(actual, expected);
    }
}
