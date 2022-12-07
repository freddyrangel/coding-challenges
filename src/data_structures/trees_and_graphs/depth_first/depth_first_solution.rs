/*
 * DEPTH-FIRST TREE TRAVERSALS
 * https://btholt.github.io/complete-intro-to-computer-science/depth-first-tree-traversals
 * Write a Depth-first tree traversal algorithm
 */
use serde::{Deserialize, Serialize};
use serde_json::value::Map;
use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;

type ChildNode = Option<Box<BSTNode>>;

#[derive(Deserialize, Debug)]
struct BSTNode {
    value: i32,
    left: ChildNode,
    right: ChildNode,
}

impl BSTNode {
    fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }
}

fn preorder_traverse(node: BSTNode) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    result.push(node.value);

    if let Some(left) = node.left {
        result.append(&mut preorder_traverse(*left));
    }

    if let Some(right) = node.right {
        result.append(&mut preorder_traverse(*right));
    }

    result
}

fn inorder_traverse(node: BSTNode) -> Vec<i32> {
    if node.is_leaf() {
        vec![node.value]
    } else {
        let mut result = vec![];

        let mut left_values = if let Some(left) = node.left {
            inorder_traverse(*left)
        } else {
            vec![]
        };

        let mut right_values = if let Some(right) = node.right {
            inorder_traverse(*right)
        } else {
            vec![]
        };

        result.append(&mut left_values);
        result.push(node.value);
        result.append(&mut right_values);

        result
    }
}

fn postorder_traverse(node: BSTNode) -> Vec<i32> {
    if node.is_leaf() {
        vec![node.value]
    } else {
        let mut result = vec![];

        let mut left_values = if let Some(left) = node.left {
            postorder_traverse(*left)
        } else {
            vec![]
        };

        let mut right_values = if let Some(right) = node.right {
            postorder_traverse(*right)
        } else {
            vec![]
        };

        result.append(&mut left_values);
        result.append(&mut right_values);
        result.push(node.value);

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn get_tree() -> BSTNode {
        let file = File::open("./src/trees/depth_first/tree.json").unwrap();
        let tree: BSTNode = serde_json::from_reader(file)
            .expect("JSON file not well-formatted or couldn't be found");

        tree
    }

    #[test]
    fn should_preorder_traverse() {
        let actual = preorder_traverse(get_tree());
        let expected = vec![8, 4, 3, 2, 5, 7, 6, 12, 10, 9, 11];

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_inorder_traverse() {
        let actual = inorder_traverse(get_tree());
        let expected = vec![2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_postorder_traverse() {
        let actual = postorder_traverse(get_tree());
        let expected = vec![2, 3, 6, 7, 5, 4, 9, 11, 10, 12, 8];

        assert_eq!(actual, expected);
    }
}
