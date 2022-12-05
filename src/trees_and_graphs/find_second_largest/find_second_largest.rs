/*
 * SECOND LARGEST IN BST
 * https://www.interviewcake.com/question/javascript/second-largest-item-in-bst
 * Find the second largest item in the binary search tree
 */

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn full_tree() {
    // let mut tree_root = BinaryTree::new(50);
    // let left_node = tree_root.insert_left(30);
    // left_node.borrow_mut().insert_left(10);
    // left_node.borrow_mut().insert_right(40);
    // let right_node = tree_root.insert_right(70);
    // right_node.borrow_mut().insert_left(60);
    // right_node.borrow_mut().insert_right(80);

    // assert_eq!(tree_root.find_second_largest(), Some(70));
    // }

    // #[test]
    // fn largest_has_a_left_child() {
    // let mut tree_root = BinaryTree::new(50);
    // let left_node = tree_root.insert_left(30);
    // left_node.borrow_mut().insert_left(10);
    // left_node.borrow_mut().insert_right(40);
    // let right_node = tree_root.insert_right(70);
    // right_node.borrow_mut().insert_left(60);

    // assert_eq!(tree_root.find_second_largest(), Some(60));
    // }

    // #[test]
    // fn largest_has_a_left_subtree() {
    // let mut tree_root = BinaryTree::new(50);
    // let mut left_node = tree_root.insert_left(30);
    // left_node.borrow_mut().insert_left(10);
    // left_node.borrow_mut().insert_right(40);
    // let right_node = tree_root.insert_right(70);
    // left_node = right_node.borrow_mut().insert_left(60);
    // left_node.borrow_mut().insert_right(65);
    // left_node
    // .borrow_mut()
    // .insert_left(55)
    // .borrow_mut()
    // .insert_right(58);

    // assert_eq!(tree_root.find_second_largest(), Some(65));
    // }

    // #[test]
    // fn second_largest_is_root_node() {
    // let mut tree_root = BinaryTree::new(50);
    // let left_node = tree_root.insert_left(30);
    // left_node.borrow_mut().insert_left(10);
    // left_node.borrow_mut().insert_right(40);
    // tree_root.insert_right(70);

    // assert_eq!(tree_root.find_second_largest(), Some(50));
    // }

    // #[test]
    // fn two_nodes_root_is_largest() {
    // let mut tree_root = BinaryTree::new(50);
    // tree_root.insert_left(30);

    // assert_eq!(tree_root.find_second_largest(), Some(30));
    // }

    // #[test]
    // fn second_largest_in_right_offshoot_left_subtree() {
    // let mut tree_root = BinaryTree::new(50);
    // let left_node = tree_root.insert_left(30);
    // left_node.borrow_mut().insert_right(40);
    // left_node.borrow_mut().insert_left(10);

    // assert_eq!(tree_root.find_second_largest(), Some(40));
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

    // assert_eq!(tree_root.find_second_largest(), Some(40));
    // }

    // #[test]
    // fn ascending_linked_list() {
    // let mut tree_root = BinaryTree::new(50);
    // tree_root
    // .insert_right(60)
    // .borrow_mut()
    // .insert_right(70)
    // .borrow_mut()
    // .insert_right(80);

    // assert_eq!(tree_root.find_second_largest(), Some(70));
    // }

    // #[test]
    // fn one_node_tree() {
    // let tree_root = BinaryTree::new(50);

    // assert_eq!(tree_root.find_second_largest(), None);
    // }
}
