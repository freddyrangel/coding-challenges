/*
 * BREADTH-FIRST TREE TRAVERSAL
 * https://btholt.github.io/complete-intro-to-computer-science/breadth-first-tree-traversals
 * Write a Breadth-first tree traversal algorithm
 */

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::VecDeque;
use std::fs::File;

type ChildNode = Option<Box<BSTNode>>;

#[derive(Deserialize, Debug)]
struct BSTNode {
    value: String,
    left: ChildNode,
    right: ChildNode,
}

fn breadth_first_traverse(tree_root: BSTNode) -> Vec<String> {
    let mut queue = VecDeque::from([tree_root]);
    let mut answer = vec![];

    while !queue.is_empty() {
        let node = queue.pop_front();

        if let Some(n) = node {
            answer.push(n.value);

            if let Some(l) = n.left {
                queue.push_back(*l);
            }

            if let Some(r) = n.right {
                queue.push_back(*r);
            }
        }
    }

    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_traverse() {
        let file = File::open("./src/trees/breadth_first/tree.json").unwrap();
        let tree: BSTNode = serde_json::from_reader(file)
            .expect("JSON file not well-formatted or couldn't be found");

        let expected = vec!["A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K"];

        assert_eq!(breadth_first_traverse(tree), expected);
    }
}
