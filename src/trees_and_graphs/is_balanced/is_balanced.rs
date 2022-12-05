/*
 * BALANCED BINARY TREE
 * https://www.interviewcake.com/question/javascript/balanced-binary-tree
 * Determine if the tree is superbalanced
 */

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn full_tree() {
    // let mut tree_root = BinaryTree::new(5);
    // let left_node = tree_root.insert_left(8);
    // left_node.borrow_mut().insert_left(1);
    // left_node.borrow_mut().insert_right(2);
    // let right_node = tree_root.insert_right(6);
    // right_node.borrow_mut().insert_left(3);
    // right_node.borrow_mut().insert_right(4);

    // assert_eq!(tree_root.is_balanced(), true);
    // }

    // #[test]
    // fn both_leaves_at_the_same_depth() {
    // let mut tree_root = BinaryTree::new(3);
    // let left_node = tree_root.insert_left(4);
    // left_node.borrow_mut().insert_left(1);
    // let right_node = tree_root.insert_right(6);
    // right_node.borrow_mut().insert_right(9);

    // assert_eq!(tree_root.is_balanced(), true);
    // }

    // #[test]
    // fn leaf_heights_differ_by_one() {
    // let mut tree_root = BinaryTree::new(6);
    // tree_root.insert_left(1);
    // let right_node = tree_root.insert_right(0);
    // right_node.borrow_mut().insert_right(7);

    // assert_eq!(tree_root.is_balanced(), true);
    // }

    // #[test]
    // fn leaf_heights_differ_by_two() {
    // let mut tree_root = BinaryTree::new(6);
    // tree_root.insert_left(1);
    // let right_node = tree_root.insert_right(0);
    // right_node
    // .borrow_mut()
    // .insert_right(7)
    // .borrow_mut()
    // .insert_right(8);

    // assert_eq!(tree_root.is_balanced(), false);
    // }

    // #[test]
    // fn three_leaves_total() {
    // let mut tree_root = BinaryTree::new(1);
    // tree_root.insert_left(5);
    // let right_node = tree_root.insert_right(9);
    // right_node.borrow_mut().insert_left(8);
    // right_node.borrow_mut().insert_right(5);

    // assert_eq!(tree_root.is_balanced(), true);
    // }

    // #[test]
    // fn both_subtrees_superbalanced() {
    // let mut tree_root = BinaryTree::new(1);
    // tree_root.insert_left(5);
    // let right_node = tree_root.insert_right(9);
    // right_node
    // .borrow_mut()
    // .insert_left(8)
    // .borrow_mut()
    // .insert_left(7);
    // right_node.borrow_mut().insert_right(5);

    // assert_eq!(tree_root.is_balanced(), false);
    // }

    // #[test]
    // fn both_subtrees_superbalanced_two() {
    // let mut tree_root = BinaryTree::new(1);
    // let left_node = tree_root.insert_left(2);
    // left_node.borrow_mut().insert_left(3);
    // left_node
    // .borrow_mut()
    // .insert_right(7)
    // .borrow_mut()
    // .insert_right(8);
    // tree_root
    // .insert_right(4)
    // .borrow_mut()
    // .insert_right(5)
    // .borrow_mut()
    // .insert_right(6)
    // .borrow_mut()
    // .insert_right(9);

    // assert_eq!(tree_root.is_balanced(), false);
    // }

    // #[test]
    // fn three_leaves_at_different_levels() {
    // let mut tree_root = BinaryTree::new(1);
    // let left_node = tree_root.insert_left(2);
    // let left_left = left_node.borrow_mut().insert_left(3);
    // left_node.borrow_mut().insert_right(4);
    // left_left.borrow_mut().insert_left(5);
    // left_left.borrow_mut().insert_right(6);
    // tree_root
    // .insert_right(7)
    // .borrow_mut()
    // .insert_right(8)
    // .borrow_mut()
    // .insert_right(9)
    // .borrow_mut()
    // .insert_right(10);

    // assert_eq!(tree_root.is_balanced(), false);
    // }

    // #[test]
    // fn only_one_node() {
    // let tree_root = BinaryTree::new(1);

    // assert_eq!(tree_root.is_balanced(), true);
    // }

    // #[test]
    // fn linked_list_tree() {
    // let mut tree_root = BinaryTree::new(1);
    // tree_root
    // .insert_right(2)
    // .borrow_mut()
    // .insert_right(3)
    // .borrow_mut()
    // .insert_right(4);

    // assert_eq!(tree_root.is_balanced(), true);
    // }
}
