/*
 * ARRAY LIST
 * https://btholt.github.io/complete-intro-to-computer-science/arraylist
 * Create an array/vector without using arrays or vectors
 */

#[cfg(test)]
mod tests {
    use super::*;

    // fn abc_range(length: u32) -> Vec<String> {
    // (0..length)
    // .map(|num| char::from_u32(97 + num).unwrap().to_string())
    // .collect()
    // }

    // #[test]
    // fn push() {
    // let mut list = ArrayList::new();
    // for character in abc_range(26) {
    // list.push(character);
    // }

    // assert_eq!(list.len(), 26);
    // }

    // #[test]
    // fn pop() {
    // let mut list = ArrayList::new();
    // for character in abc_range(13) {
    // list.push(character);
    // }

    // assert_eq!(list.len(), 13);

    // for _ in 0..10 {
    // list.pop();
    // }

    // assert_eq!(list.len(), 3);
    // assert_eq!(list.pop(), "c");
    // }

    // #[test]
    // fn get() {
    // let mut list = ArrayList::new();
    // list.push("first".to_string());
    // assert_eq!(list.get(0), "first");
    // list.push("second".to_string());
    // assert_eq!(list.get(1), "second".to_string());
    // assert_eq!(list.get(0), "first".to_string());

    // for character in abc_range(26) {
    // list.push(character);
    // }

    // assert_eq!(list.get(27), "z");
    // assert_eq!(list.get(0), "first");
    // assert_eq!(list.get(9), "h");

    // list.pop();

    // assert_eq!(list.get(list.len() - 1), "y");
    // }

    // #[test]
    // fn delete() {
    // let mut list = ArrayList::new();
    // for character in abc_range(26) {
    // list.push(character);
    // }

    // list.delete(13);

    // assert_eq!(list.len(), 25);
    // assert_eq!(list.get(12), "m");
    // assert_eq!(list.get(13), "o");

    // list.delete(0);

    // assert_eq!(list.len(), 24);
    // assert_eq!(list.get(0), "b");
    // }
}
