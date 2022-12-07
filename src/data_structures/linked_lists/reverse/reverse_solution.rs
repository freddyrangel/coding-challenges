/*
 * REVERSE
 * https://www.interviewcake.com/question/javascript/reverse-linked-list
 * Reverse the linked list in place
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

#[derive(Debug)]
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

impl<T: Debug + Clone> LinkedList<T> {
    fn reverse(&mut self) {
        let mut current_node_option = self.head.clone();
        let mut previous_node_option = None;
        let mut _next_node_option = None;

        while let Some(current_node) = &current_node_option {
            _next_node_option = current_node.clone().borrow().next.clone();

            if _next_node_option.is_none() {
                self.head = current_node_option.clone();
            }

            current_node.clone().borrow_mut().next = previous_node_option;

            previous_node_option = current_node_option.clone();

            current_node_option = _next_node_option;
        }
    }

    fn values(&self) -> Vec<T> {
        let nodes = self.nodes();

        nodes
            .iter()
            .map(|node| node.borrow().value.clone())
            .collect()
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
    fn short_linked_list() {
        let mut list = LinkedList::from([1, 2]);
        list.reverse();
        let values = list.values();
        assert_eq!(values, vec![2, 1]);
    }

    #[test]
    fn long_linked_list() {
        let mut list = LinkedList::from([1, 2, 3, 4, 5, 6]);
        list.reverse();
        let values = list.values();
        assert_eq!(values, vec![6, 5, 4, 3, 2, 1]);
    }

    #[test]
    fn one_element_linked_list() {
        let mut list = LinkedList::from([1]);
        list.reverse();
        let values = list.values();
        assert_eq!(values, vec![1]);
    }

    #[test]
    fn empty_linked_list() {
        let mut list: LinkedList<usize> = LinkedList::from([]);
        list.reverse();

        let values = list.values();
        assert_eq!(values, vec![]);
    }
}
