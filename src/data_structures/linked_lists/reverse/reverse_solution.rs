/*
 * REVERSE
 * https://www.interviewcake.com/question/javascript/reverse-linked-list
 * Reverse the singly linked list in place
 */

use std::cell::RefCell;
use std::rc::Rc;

struct LinkedList<T> {
    head: OptionNode<T>,
    tail: OptionNode<T>,
}

type OptionNode<T> = Option<RefNode<T>>;
type RefNode<T> = Rc<RefCell<Node<T>>>;

struct Node<T> {
    value: T,
    next: OptionNode<T>,
}

impl<T: Copy, const N: usize> From<[T; N]> for LinkedList<T> {
    fn from(values: [T; N]) -> Self {
        let mut list = LinkedList::new();

        for value in values {
            let node = Some(Node::new(value));
            match list.tail.clone() {
                Some(tail) => {
                    tail.borrow_mut().next = node.clone();
                    list.tail = node.clone();
                }
                None => {
                    list.head = node.clone();
                    list.tail = node.clone();
                }
            }
        }

        list
    }
}

impl<T: Copy> LinkedList<T> {
    fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
        }
    }

    fn reverse(&mut self) {
        let mut values = self.values();
        values.reverse();
        let mut current_node = self.head.clone();

        for value in values {
            if let Some(node) = current_node.clone() {
                node.borrow_mut().value = value;
                current_node = node.borrow().next.clone();
            }
        }
    }

    fn values(&self) -> Vec<T> {
        let mut values = Vec::new();
        let mut current_node = self.head.clone();

        while let Some(node) = current_node {
            values.push(node.borrow().value);
            current_node = node.borrow().next.clone();
        }

        values
    }
}

impl<T> Node<T> {
    fn new(value: T) -> RefNode<T> {
        Rc::new(RefCell::new(Node { value, next: None }))
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
