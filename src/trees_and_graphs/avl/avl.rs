/*
 * AVL TREE
 * https://btholt.github.io/complete-intro-to-computer-science/avl-tree
 * Create an AVL Tree
 */

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn it_creates_avl_tree() {
    // let nums = vec![3, 7, 4, 6, 5, 1, 10, 2, 9, 8];
    // let mut tree = AVLTree::new();

    // nums.iter().for_each(|num| tree.add(*num));
    // println!("TREE: {:?}", tree);

    // let head = tree.head.unwrap();

    // assert_eq!(head.borrow().value, 4);

    // let node = head.clone().borrow().left.clone().unwrap();

    // assert_eq!(node.borrow().value, 2);

    //assert_eq!(head.left.as_ref().unwrap().left.as_ref().unwrap().value, 1);
    //assert_eq!(
    //head.left
    //.as_ref()
    //.unwrap()
    //.left
    //.as_ref()
    //.unwrap()
    //.left
    //.as_ref()
    //.is_none(),
    //true
    //);
    //assert_eq!(
    //head.left
    //.as_ref()
    //.unwrap()
    //.left
    //.as_ref()
    //.unwrap()
    //.right
    //.as_ref()
    //.is_none(),
    //true
    //);

    //assert_eq!(head.left.as_ref().unwrap().right.as_ref().unwrap().value, 3);
    //assert_eq!(
    //head.left
    //.as_ref()
    //.unwrap()
    //.right
    //.as_ref()
    //.unwrap()
    //.left
    //.as_ref()
    //.is_none(),
    //true
    //);
    //assert_eq!(
    //head.left
    //.as_ref()
    //.unwrap()
    //.right
    //.as_ref()
    //.unwrap()
    //.right
    //.as_ref()
    //.is_none(),
    //true
    //);

    //assert_eq!(head.right.as_ref().unwrap().value, 7);

    //assert_eq!(head.right.as_ref().unwrap().left.as_ref().unwrap().value, 6);
    //assert_eq!(
    //head.right
    //.as_ref()
    //.unwrap()
    //.left
    //.as_ref()
    //.unwrap()
    //.right
    //.as_ref()
    //.is_none(),
    //true
    //);

    //assert_eq!(
    //head.right
    //.as_ref()
    //.unwrap()
    //.left
    //.as_ref()
    //.unwrap()
    //.left
    //.as_ref()
    //.unwrap()
    //.value,
    //5
    //);
    //assert_eq!(
    //head.right
    //.as_ref()
    //.unwrap()
    //.left
    //.as_ref()
    //.unwrap()
    //.left
    //.as_ref()
    //.unwrap()
    //.left
    //.as_ref()
    //.is_none(),
    //true
    //);
    //assert_eq!(
    //head.right
    //.as_ref()
    //.unwrap()
    //.left
    //.as_ref()
    //.unwrap()
    //.left
    //.as_ref()
    //.unwrap()
    //.right
    //.as_ref()
    //.is_none(),
    //true
    //);

    //assert_eq!(
    //head.right.as_ref().unwrap().right.as_ref().unwrap().value,
    //9
    //);

    //assert_eq!(
    //head.right
    //.as_ref()
    //.unwrap()
    //.right
    //.as_ref()
    //.unwrap()
    //.left
    //.as_ref()
    //.unwrap()
    //.value,
    //8
    //);
    //assert_eq!(
    //head.right
    //.as_ref()
    //.unwrap()
    //.right
    //.as_ref()
    //.unwrap()
    //.left
    //.as_ref()
    //.unwrap()
    //.left
    //.as_ref()
    //.is_none(),
    //true
    //);
    //assert_eq!(
    //head.right
    //.as_ref()
    //.unwrap()
    //.right
    //.as_ref()
    //.unwrap()
    //.left
    //.as_ref()
    //.unwrap()
    //.right
    //.as_ref()
    //.is_none(),
    //true
    //);

    //assert_eq!(
    //head.right
    //.as_ref()
    //.unwrap()
    //.right
    //.as_ref()
    //.unwrap()
    //.right
    //.as_ref()
    //.unwrap()
    //.value,
    //10
    //);
    //assert_eq!(
    //head.right
    //.as_ref()
    //.unwrap()
    //.right
    //.as_ref()
    //.unwrap()
    //.right
    //.as_ref()
    //.unwrap()
    //.left
    //.as_ref()
    //.is_none(),
    //true
    //);
    //assert_eq!(
    //head.right
    //.as_ref()
    //.unwrap()
    //.right
    //.as_ref()
    //.unwrap()
    //.right
    //.as_ref()
    //.unwrap()
    //.right
    //.as_ref()
    //.is_none(),
    //true
    //);
    // }
}
