// Implement the push, pop, and get_max methods
// https://www.interviewcake.com/question/javascript/largest-stack

#![allow(dead_code, unused_imports)]

type ChildStack = Box<Stack>;

struct MaxStack {
    stack: ChildStack,
    max_stack: ChildStack,
}

impl MaxStack {
    fn new() -> Self {
        MaxStack {
            stack: Box::new(Stack::new()),
            max_stack: Box::new(Stack::new()),
        }
    }

    fn push(&mut self, item: usize) {
        if item >= self.max_stack.peek() {
            self.max_stack.push(item);
        }

        self.stack.push(item);
    }

    fn get_max(&self) -> usize {
        self.max_stack.peek()
    }

    fn pop(&mut self) -> usize {
        let item = self.stack.pop();

        if item == self.max_stack.peek() {
            self.max_stack.pop();
        }

        item
    }
}

struct Stack {
    items: Vec<usize>,
}

impl Stack {
    fn new() -> Self {
        Stack { items: Vec::new() }
    }

    fn push(&mut self, item: usize) {
        self.items.push(item);
    }

    fn pop(&mut self) -> usize {
        self.items.pop().unwrap()
    }

    fn peek(&self) -> usize {
        match self.items.last() {
            Some(i) => *i,
            None => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_max_stack() -> MaxStack {
        let mut s = MaxStack::new();
        s.push(5);
        s.push(4);
        s.push(7);
        s.push(7);
        s.push(8);
        s
    }

    #[test]
    fn check_max_after_first_push() {
        let mut s = MaxStack::new();
        s.push(5);
        assert_eq!(5, s.get_max());
    }

    #[test]
    fn check_before_first_pop() {
        let s = setup_max_stack();
        assert_eq!(8, s.get_max());
    }

    #[test]
    fn check_first_pop() {
        let mut s = setup_max_stack();
        assert_eq!(8, s.pop());
    }

    #[test]
    fn check_max_after_first_pop() {
        let mut s = setup_max_stack();
        s.pop();
        assert_eq!(7, s.get_max());
    }

    #[test]
    fn check_second_pop() {
        let mut s = setup_max_stack();
        s.pop();
        assert_eq!(7, s.pop());
    }

    #[test]
    fn check_max_after_second_pop() {
        let mut s = setup_max_stack();
        s.pop();
        s.pop();
        assert_eq!(7, s.get_max());
    }

    #[test]
    fn check_third_pop() {
        let mut s = setup_max_stack();
        s.pop();
        s.pop();
        assert_eq!(7, s.pop());
    }

    #[test]
    fn check_max_after_third_pop() {
        let mut s = setup_max_stack();
        s.pop();
        s.pop();
        s.pop();
        assert_eq!(5, s.get_max());
    }

    #[test]
    fn check_fourth_pop() {
        let mut s = setup_max_stack();
        s.pop();
        s.pop();
        s.pop();
        assert_eq!(4, s.pop());
    }

    #[test]
    fn check_max_after_fourth_pop() {
        let mut s = setup_max_stack();
        s.pop();
        s.pop();
        s.pop();
        s.pop();
        assert_eq!(5, s.get_max());
    }
}
