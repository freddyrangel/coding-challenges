/*
 * BINARY SEARCH TREE
 * https://btholt.github.io/complete-intro-to-computer-science/binary-search-tree
 * Create a Binary Search Tree
 */

type ChildNode = Option<Box<BSTNode>>;

#[derive(Debug)]
struct BST {
    head: ChildNode,
}

#[derive(Debug)]
struct BSTNode {
    value: i32,
    left: ChildNode,
    right: ChildNode,
}

impl BST {
    fn new() -> Self {
        BST { head: None }
    }

    fn add(&mut self, node: BSTNode) {
        let head = &mut self.head;

        match head {
            Some(h) => {
                h.add(node);
            }
            None => {
                self.head = Some(Box::new(node));
            }
        }
    }
}

impl BSTNode {
    fn new(value: i32) -> Self {
        BSTNode {
            value,
            left: None,
            right: None,
        }
    }

    fn add(&mut self, node: BSTNode) {
        let current_value = self.value;
        let other_value = node.value;

        if other_value <= current_value {
            let left = &mut self.left;

            match left {
                Some(l) => {
                    l.add(node);
                }
                None => {
                    self.left = Some(Box::new(node));
                }
            }
        } else {
            let right = &mut self.right;

            match right {
                Some(r) => {
                    r.add(node);
                }
                None => {
                    self.right = Some(Box::new(node));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_a_correct_tree() {
        let nums = vec![3, 7, 4, 6, 5, 1, 10, 2, 9, 8];
        let mut tree = BST::new();
        for num in &nums {
            tree.add(BSTNode::new(num.clone()));
        }

        let head = tree.head.unwrap();

        assert_eq!(head.value, 3);
        assert_eq!(head.left.as_ref().unwrap().value, 1);
        assert_eq!(head.left.as_ref().unwrap().left.is_none(), true);

        assert_eq!(head.left.as_ref().unwrap().right.as_ref().unwrap().value, 2);
        assert_eq!(
            head.left
                .as_ref()
                .unwrap()
                .right
                .as_ref()
                .unwrap()
                .left
                .is_none(),
            true
        );

        assert_eq!(
            head.left
                .as_ref()
                .unwrap()
                .right
                .as_ref()
                .unwrap()
                .left
                .is_none(),
            true
        );
        assert_eq!(
            head.left
                .as_ref()
                .unwrap()
                .right
                .as_ref()
                .unwrap()
                .right
                .is_none(),
            true
        );

        assert_eq!(head.right.as_ref().unwrap().value, 7);

        assert_eq!(head.right.as_ref().unwrap().left.as_ref().unwrap().value, 4);
        assert_eq!(
            head.right
                .as_ref()
                .unwrap()
                .left
                .as_ref()
                .unwrap()
                .left
                .is_none(),
            true
        );

        assert_eq!(
            head.right
                .as_ref()
                .unwrap()
                .left
                .as_ref()
                .unwrap()
                .right
                .as_ref()
                .unwrap()
                .value,
            6
        );
        assert_eq!(
            head.right
                .as_ref()
                .unwrap()
                .left
                .as_ref()
                .unwrap()
                .right
                .as_ref()
                .unwrap()
                .left
                .as_ref()
                .unwrap()
                .value,
            5
        );
        assert_eq!(
            head.right
                .as_ref()
                .unwrap()
                .left
                .as_ref()
                .unwrap()
                .right
                .as_ref()
                .unwrap()
                .left
                .as_ref()
                .unwrap()
                .right
                .is_none(),
            true
        );
        assert_eq!(
            head.right
                .as_ref()
                .unwrap()
                .left
                .as_ref()
                .unwrap()
                .right
                .as_ref()
                .unwrap()
                .left
                .as_ref()
                .unwrap()
                .left
                .is_none(),
            true
        );

        assert_eq!(
            head.right.as_ref().unwrap().right.as_ref().unwrap().value,
            10
        );
        assert_eq!(
            head.right
                .as_ref()
                .unwrap()
                .right
                .as_ref()
                .unwrap()
                .right
                .is_none(),
            true
        );

        assert_eq!(
            head.right
                .as_ref()
                .unwrap()
                .right
                .as_ref()
                .unwrap()
                .left
                .as_ref()
                .unwrap()
                .value,
            9
        );
        assert_eq!(
            head.right
                .as_ref()
                .unwrap()
                .right
                .as_ref()
                .unwrap()
                .left
                .as_ref()
                .unwrap()
                .right
                .is_none(),
            true
        );

        assert_eq!(
            head.right
                .as_ref()
                .unwrap()
                .right
                .as_ref()
                .unwrap()
                .left
                .as_ref()
                .unwrap()
                .left
                .as_ref()
                .unwrap()
                .value,
            8
        );
        assert_eq!(
            head.right
                .as_ref()
                .unwrap()
                .right
                .as_ref()
                .unwrap()
                .left
                .as_ref()
                .unwrap()
                .left
                .as_ref()
                .unwrap()
                .right
                .is_none(),
            true
        );
        assert_eq!(
            head.right
                .as_ref()
                .unwrap()
                .right
                .as_ref()
                .unwrap()
                .left
                .as_ref()
                .unwrap()
                .left
                .as_ref()
                .unwrap()
                .left
                .is_none(),
            true
        );
    }
}
