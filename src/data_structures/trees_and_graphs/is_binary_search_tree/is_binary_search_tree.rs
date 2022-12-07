/*
 * BST CHECKER
 * https://www.interviewcake.com/question/javascript/bst-checker
 * Determine if the tree is a valid binary search tree
 */

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn valid_full_tree() {
    // let mut tree_root = BinaryTree::new(50);
    // let left_node = tree_root.insert_left(30);
    // left_node.borrow_mut().insert_left(10);
    // left_node.borrow_mut().insert_right(40);
    // let right_node = tree_root.insert_right(70);
    // right_node.borrow_mut().insert_left(60);
    // right_node.borrow_mut().insert_right(80);

    // assert_eq!(tree_root.is_binary_search_tree(), true);
    // }

    // #[test]
    // fn both_subtrees_valid() {
    // let mut tree_root = BinaryTree::new(50);
    // let left_node = tree_root.insert_left(30);
    // left_node.borrow_mut().insert_left(20);
    // left_node.borrow_mut().insert_right(60);
    // let right_node = tree_root.insert_right(80);
    // right_node.borrow_mut().insert_left(70);
    // right_node.borrow_mut().insert_right(90);

    // assert_eq!(tree_root.is_binary_search_tree(), false);
    // }

    // #[test]
    // fn descending_linked_list() {
    // let mut tree_root = BinaryTree::new(50);
    // tree_root
    // .insert_left(40)
    // .borrow_mut()
    // .insert_left(30)
    // .borrow_mut()
    // .insert_left(20)
    // .borrow_mut()
    // .insert_left(10);

    // assert_eq!(tree_root.is_binary_search_tree(), true);
    // }

    // #[test]
    // fn out_of_order_linked_list() {
    // let mut tree_root = BinaryTree::new(50);
    // tree_root
    // .insert_right(70)
    // .borrow_mut()
    // .insert_right(60)
    // .borrow_mut()
    // .insert_right(80);

    // assert_eq!(tree_root.is_binary_search_tree(), false);
    // }

    // #[test]
    // fn one_node_tree() {
    // let tree_root = BinaryTree::new(50);

    // assert_eq!(tree_root.is_binary_search_tree(), true);
    // }
}
