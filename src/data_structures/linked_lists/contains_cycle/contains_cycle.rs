/*
 * CONTAINS CYCLE
 * https://www.interviewcake.com/question/javascript/linked-list-cycles
 * Check if the linked list contains a cycle
 */

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn linked_list_with_no_cycle() {
    // let list = LinkedList::from([1, 2, 3, 4]);

    // let mut node = Rc::clone(&list.head.as_ref().unwrap());
    // assert_eq!(1, node.borrow().value);
    // node = Rc::clone(&node).borrow().next.clone().unwrap();
    // assert_eq!(2, node.borrow().value);
    // node = Rc::clone(&node).borrow().next.clone().unwrap();
    // assert_eq!(3, node.borrow().value);
    // node = Rc::clone(&node).borrow().next.clone().unwrap();
    // assert_eq!(4, node.borrow().value);
    // assert_eq!(None, Rc::clone(&node).borrow().next);

    // assert_eq!(list.contains_cycle(), false);
    // }
    // #[test]
    // fn cycle_loops_to_beginning() {
    // let list = LinkedList::from([1, 2, 3, 4]);
    // let first_node = Rc::clone(&list.head.as_ref().unwrap());
    // let mut last_node = Rc::clone(&first_node).borrow().next.clone().unwrap();
    // last_node = Rc::clone(&last_node).borrow().next.clone().unwrap();
    // last_node = Rc::clone(&last_node).borrow().next.clone().unwrap();

    // Rc::clone(&last_node).borrow_mut().next = Some(first_node);

    // assert_eq!(list.contains_cycle(), true);
    // }

    // #[test]
    // fn cycle_loops_to_middle() {
    // let list = LinkedList::from([1, 2, 3, 4, 5]);
    // let mut second_node = Rc::clone(&list.head.as_ref().unwrap());
    // second_node = Rc::clone(&second_node).borrow().next.clone().unwrap();

    // let mut last_node = Rc::clone(&second_node).borrow().next.clone().unwrap();
    // last_node = Rc::clone(&last_node).borrow().next.clone().unwrap();
    // last_node = Rc::clone(&last_node).borrow().next.clone().unwrap();

    // Rc::clone(&last_node).borrow_mut().next = Some(second_node);

    // assert_eq!(list.contains_cycle(), true);
    // }

    // #[test]
    // fn two_node_cycle_at_end() {
    // let list = LinkedList::from([1, 2, 3, 4, 5]);
    // let mut third_node = Rc::clone(&list.head.as_ref().unwrap());
    // third_node = Rc::clone(&third_node).borrow().next.clone().unwrap();
    // third_node = Rc::clone(&third_node).borrow().next.clone().unwrap();

    // let last_node = Rc::clone(&third_node).borrow().next.clone().unwrap();

    // Rc::clone(&last_node).borrow_mut().next = Some(third_node);

    // assert_eq!(list.contains_cycle(), true);
    // }

    // #[test]
    // fn empty_list() {
    // let list: LinkedList<usize> = LinkedList::from([]);

    // assert_eq!(list.contains_cycle(), false);
    // }

    // #[test]
    // fn one_element_linked_list_no_cycle() {
    // let list = LinkedList::from([1]);

    // assert_eq!(list.contains_cycle(), false);
    // }
}
