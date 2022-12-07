/*
 * ARRAY LIST
 * https://btholt.github.io/complete-intro-to-computer-science/arraylist
 * Create an array/vector without using arrays or vectors
 */

use std::collections::HashMap;
use std::fmt::Debug;

struct ArrayList<T> {
    length: usize,
    elements: HashMap<usize, T>,
}

impl<T: Clone + Debug> ArrayList<T> {
    fn new() -> Self {
        Self {
            length: 0,
            elements: HashMap::new(),
        }
    }

    fn push(&mut self, value: T) {
        self.elements.insert(self.length, value);
        self.length += 1;
    }

    fn len(&self) -> usize {
        self.length
    }

    fn pop(&mut self) -> T {
        self.length -= 1;
        self.elements.remove(&self.length).unwrap()
    }

    fn get(&self, index: usize) -> T {
        let result = self.elements.get(&index).unwrap().clone();

        result
    }

    fn delete(&mut self, index: usize) {
        self.length -= 1;
        self.elements.remove(&index).unwrap();

        for i in index..self.length {
            let next = self.get(i + 1);
            self.elements.insert(i, next);
        }
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
        let mut list = ArrayList::new();
        for character in abc_range(26) {
            list.push(character);
        }

        assert_eq!(list.len(), 26);
    }

    #[test]
    fn pop() {
        let mut list = ArrayList::new();
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
        let mut list = ArrayList::new();
        list.push("first".to_string());
        assert_eq!(list.get(0), "first");
        list.push("second".to_string());
        assert_eq!(list.get(1), "second".to_string());
        assert_eq!(list.get(0), "first".to_string());

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
        let mut list = ArrayList::new();
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
