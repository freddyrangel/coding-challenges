/*
 * QUEUE TWO STACKS
 * https://www.interviewcake.com/question/javascript/queue-two-stacks
 * Implement the enqueue and dequeue methods
 */

use std::collections::VecDeque;

type ChildStack = Box<Stack>;

struct QueueTwoStacks {
    stack_one: ChildStack,
    stack_two: ChildStack,
}

impl QueueTwoStacks {
    fn new() -> Self {
        QueueTwoStacks {
            stack_one: Box::new(Stack::new()),
            stack_two: Box::new(Stack::new()),
        }
    }

    fn enqueue(&mut self, item: usize) {
        self.stack_one.push(item);
        self.stack_two.push(item);
    }

    fn dequeue(&mut self) -> Option<usize> {
        let item = self.stack_one.pop_front();
        self.stack_two.pop_front();

        item
    }
}

struct Stack {
    items: VecDeque<usize>,
}

impl Stack {
    fn new() -> Self {
        Stack {
            items: VecDeque::new(),
        }
    }

    fn push(&mut self, item: usize) {
        self.items.push_back(item);
    }

    fn pop_front(&mut self) -> Option<usize> {
        self.items.pop_front()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_queue() -> QueueTwoStacks {
        let mut q = QueueTwoStacks::new();

        q.enqueue(1);
        q.enqueue(2);
        q.enqueue(3);

        q
    }

    #[test]
    fn dequeue_first() {
        let mut q = setup_queue();
        let actual = q.dequeue();
        let expected = Some(1);
        assert_eq!(actual, expected);
    }

    #[test]
    fn dequeue_second() {
        let mut q = setup_queue();
        q.dequeue();
        let actual = q.dequeue();
        let expected = Some(2);
        assert_eq!(actual, expected);
    }

    #[test]
    fn dequeue_third() {
        let mut q = setup_queue();
        q.dequeue();
        q.dequeue();
        q.enqueue(4);
        let actual = q.dequeue();
        let expected = Some(3);
        assert_eq!(actual, expected);
    }

    #[test]
    fn dequeue_fourth() {
        let mut q = setup_queue();
        q.dequeue();
        q.dequeue();
        q.enqueue(4);
        q.dequeue();
        let actual = q.dequeue();
        let expected = Some(4);
        assert_eq!(actual, expected);
    }

    #[test]
    fn dequeue_from_empty_queue() {
        let mut q = QueueTwoStacks::new();
        assert_eq!(None, q.dequeue());
    }
}
