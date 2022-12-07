/*
 * LINKED LIST
 * https://btholt.github.io/complete-intro-to-computer-science/linkedlist
 * Create a linked list
 */

use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

type OptionNode<T> = Option<RefNode<T>>;
type RefNode<T> = Rc<RefCell<Box<Node<T>>>>;

struct LinkedList<T> {
    head: OptionNode<T>,
    tail: OptionNode<T>,
    length: usize,
}

#[derive(Debug)]
struct Node<T> {
    value: T,
    previous: OptionNode<T>,
    next: OptionNode<T>,
}

impl<T: Clone + Debug> LinkedList<T> {
    fn new() -> Self {
        Self {
            head: None,
            tail: None,
            length: 0,
        }
    }

    fn push(&mut self, value: T) {
        let next_node = Node::new(value);
        if let Some(tail) = self.tail.clone() {
            tail.borrow_mut().next = Some(next_node.clone());
            next_node.borrow_mut().previous = Some(tail.clone());
            self.tail = Some(next_node.clone());
        } else {
            self.head = Some(next_node.clone());
            self.tail = Some(next_node.clone());
        }
        self.length += 1;
    }

    fn pop(&mut self) -> T {
        let last_tail = Rc::clone(&self.tail.clone().unwrap());
        let new_tail = Rc::clone(&last_tail).borrow().previous.clone().unwrap();
        let mut new_tail_ref = new_tail.borrow_mut();
        new_tail_ref.next = None;
        self.tail = Some(Rc::clone(&new_tail));
        self.length -= 1;

        let temp = last_tail.borrow();
        temp.value.clone()
    }

    fn get(&self, index: usize) -> T {
        let node = self.get_node(index).unwrap();

        let temp = node.borrow();
        temp.value.clone()
    }

    fn len(&self) -> usize {
        self.length
    }

    fn delete(&mut self, index: usize) {
        let node = self.get_node(index).unwrap().clone();
        let next = node.borrow().next.clone().unwrap();
        self.length -= 1;
        if index == 0 {
            self.head = Some(next.clone());
        } else {
            let previous = node.borrow().previous.clone().unwrap();
            previous.borrow_mut().next = Some(next.clone());
            next.borrow_mut().previous = Some(previous.clone());
        }
    }

    fn get_node(&self, index: usize) -> OptionNode<T> {
        let mut node = self.head.clone();
        for _ in 0..index {
            if let Some(n) = node {
                node = n.clone().borrow().next.clone();
            }
        }

        node
    }
}

impl<T> Node<T> {
    fn new(value: T) -> RefNode<T> {
        Rc::new(RefCell::new(Box::new(Node {
            value,
            next: None,
            previous: None,
        })))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn abc_range(length: u32) -> Vec<String> {
        (0..length)
            .map(|num| char::from_u32(97 + num).unwrap().to_string())
            .collect()
    }

    #[test]
    fn push() {
        let mut list = LinkedList::new();
        for character in abc_range(26) {
            list.push(character);
        }
        assert_eq!(list.len(), 26);
    }

    #[test]
    fn pop() {
        let mut list = LinkedList::new();
        for character in abc_range(13) {
            list.push(character);
        }
        assert_eq!(list.len(), 13);

        for _ in 0..10 {
            list.pop();
        }
        assert_eq!(list.len(), 3);
        assert_eq!(list.pop(), "c");
    }

    #[test]
    fn get() {
        let mut list = LinkedList::new();
        list.push("first".to_string());
        assert_eq!(list.get(0), "first");
        list.push("second".to_string());
        assert_eq!(list.get(1), "second");
        assert_eq!(list.get(0), "first");
        for character in abc_range(26) {
            list.push(character);
        }
        assert_eq!(list.get(27), "z");
        assert_eq!(list.get(0), "first");
        assert_eq!(list.get(9), "h");
        list.pop();
        assert_eq!(list.get(list.len() - 1), "y");
    }

    #[test]
    fn delete() {
        let mut list = LinkedList::new();
        for character in abc_range(26) {
            list.push(character);
        }
        list.delete(13);
        assert_eq!(list.len(), 25);
        assert_eq!(list.get(12), "m");
        assert_eq!(list.get(13), "o");
        list.delete(0);
        assert_eq!(list.len(), 24);
        assert_eq!(list.get(0), "b");
    }
}
