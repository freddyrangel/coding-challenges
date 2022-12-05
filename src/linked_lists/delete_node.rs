// Delete the input node from the linked list
// https://www.interviewcake.com/question/javascript/delete-node

#![allow(dead_code, unused_imports)]

use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

type RefNode<T> = Rc<RefCell<LinkedListNode<T>>>;

struct LinkedList<T> {
    head: RefNode<T>,
}

#[derive(Debug, PartialEq, Eq)]
struct LinkedListNode<T> {
    value: T,
    next: Option<RefNode<T>>,
}

impl<T: PartialEq + Debug> LinkedList<T> {
    fn new(value: T) -> Self {
        LinkedList {
            head: LinkedListNode::new(value),
        }
    }

    fn append(&mut self, value: T) -> RefNode<T> {
        let mut tail = Rc::clone(&self.head);

        while let Some(next) = &Rc::clone(&tail).borrow().next {
            tail = Rc::clone(&next);
        }

        let next = LinkedListNode::new(value);

        tail.borrow_mut().next = Some(Rc::clone(&next));

        Rc::clone(&next)
    }

    fn delete(&mut self, node: RefNode<T>) -> Result<(), &str> {
        let head = Rc::clone(&self.head);

        if head == node {
            if head.borrow().next == None {
                return Err("Can't delete the only node in this list");
            }
            if let Some(next) = &Rc::clone(&head).borrow().next {
                self.head = Rc::clone(next);
            }
        } else {
            let mut previous_node = Rc::clone(&self.head);
            let mut next_node = None;

            while let Some(current_node) = &Rc::clone(&previous_node).borrow().next {
                if current_node == &node {
                    next_node = current_node.borrow().next.clone();
                    break;
                } else {
                    previous_node = Rc::clone(current_node);
                }
            }

            previous_node.borrow_mut().next = next_node;
        }

        Ok(())
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
    fn node_at_beginning() {
        let mut list = LinkedList::new(1);
        list.append(2);
        list.append(3);
        list.append(4);

        let head = Rc::clone(&list.head);
        let _ = list.delete(head);

        let mut node = Rc::clone(&list.head);
        assert_eq!(2, node.borrow().value);
        node = Rc::clone(&node).borrow().next.clone().unwrap();
        assert_eq!(3, node.borrow().value);
        node = Rc::clone(&node).borrow().next.clone().unwrap();
        assert_eq!(4, node.borrow().value);
        assert_eq!(None, Rc::clone(&node).borrow().next);
    }

    #[test]
    fn node_in_middle() {
        let mut list = LinkedList::new(1);
        let node_to_delete = list.append(2);
        list.append(3);
        list.append(4);

        let _ = list.delete(node_to_delete);

        let mut node = Rc::clone(&list.head);
        assert_eq!(1, node.borrow().value);
        node = Rc::clone(&node).borrow().next.clone().unwrap();
        assert_eq!(3, node.borrow().value);
        node = Rc::clone(&node).borrow().next.clone().unwrap();
        assert_eq!(4, node.borrow().value);
        assert_eq!(None, Rc::clone(&node).borrow().next);
    }

    #[test]
    fn node_at_end() {
        let mut list = LinkedList::new(1);
        list.append(2);
        list.append(3);
        let node_to_delete = list.append(4);

        let _ = list.delete(node_to_delete);

        let mut node = Rc::clone(&list.head);
        assert_eq!(1, node.borrow().value);
        node = Rc::clone(&node).borrow().next.clone().unwrap();
        assert_eq!(2, node.borrow().value);
        node = Rc::clone(&node).borrow().next.clone().unwrap();
        assert_eq!(3, node.borrow().value);
        assert_eq!(None, Rc::clone(&node).borrow().next);
    }

    #[test]
    fn one_node() {
        let mut list = LinkedList::new(1);
        let head = Rc::clone(&list.head);
        let result = list.delete(head);

        assert_eq!(true, result.is_err());
    }
}
