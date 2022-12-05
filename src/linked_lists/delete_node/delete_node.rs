/*
 * DELETE LINKED LIST NODE
 * https://www.interviewcake.com/question/javascript/delete-node
 * Delete the input node from the linked list
 */

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn node_at_beginning() {
    // let mut list = LinkedList::new(1);
    // list.append(2);
    // list.append(3);
    // list.append(4);

    // let head = Rc::clone(&list.head);
    // let _ = list.delete(head);

    // let mut node = Rc::clone(&list.head);
    // assert_eq!(2, node.borrow().value);
    // node = Rc::clone(&node).borrow().next.clone().unwrap();
    // assert_eq!(3, node.borrow().value);
    // node = Rc::clone(&node).borrow().next.clone().unwrap();
    // assert_eq!(4, node.borrow().value);
    // assert_eq!(None, Rc::clone(&node).borrow().next);
    // }

    // #[test]
    // fn node_in_middle() {
    // let mut list = LinkedList::new(1);
    // let node_to_delete = list.append(2);
    // list.append(3);
    // list.append(4);

    // let _ = list.delete(node_to_delete);

    // let mut node = Rc::clone(&list.head);
    // assert_eq!(1, node.borrow().value);
    // node = Rc::clone(&node).borrow().next.clone().unwrap();
    // assert_eq!(3, node.borrow().value);
    // node = Rc::clone(&node).borrow().next.clone().unwrap();
    // assert_eq!(4, node.borrow().value);
    // assert_eq!(None, Rc::clone(&node).borrow().next);
    // }

    // #[test]
    // fn node_at_end() {
    // let mut list = LinkedList::new(1);
    // list.append(2);
    // list.append(3);
    // let node_to_delete = list.append(4);

    // let _ = list.delete(node_to_delete);

    // let mut node = Rc::clone(&list.head);
    // assert_eq!(1, node.borrow().value);
    // node = Rc::clone(&node).borrow().next.clone().unwrap();
    // assert_eq!(2, node.borrow().value);
    // node = Rc::clone(&node).borrow().next.clone().unwrap();
    // assert_eq!(3, node.borrow().value);
    // assert_eq!(None, Rc::clone(&node).borrow().next);
    // }

    // #[test]
    // fn one_node() {
    // let mut list = LinkedList::new(1);
    // let head = Rc::clone(&list.head);
    // let result = list.delete(head);

    // assert_eq!(true, result.is_err());
    // }
}
