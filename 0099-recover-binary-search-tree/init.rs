// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use std::cell::RefCell;
use std::rc::Rc;

pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
    fn traversal(
        root: &mut Option<Rc<RefCell<TreeNode>>>,
        prev: &mut Option<Rc<RefCell<TreeNode>>>,
        first: &mut Option<Rc<RefCell<TreeNode>>>,
        second: &mut Option<Rc<RefCell<TreeNode>>>,
    ) {
        if let Some(node) = root.as_ref() {
            traversal(&mut node.borrow().left.clone(), prev, first, second);
            if let Some(prev_node) = prev.as_ref() {
                if first.is_none() && prev_node.borrow().val > node.borrow().val {
                    first.replace(prev_node.clone());
                }
                if first.is_some() && prev_node.borrow().val > node.borrow().val {
                    second.replace(node.clone());
                }
            }
            prev.replace(node.clone());
            traversal(&mut node.borrow().right.clone(), prev, first, second);
        }
    }

    let mut prev: Option<Rc<RefCell<TreeNode>>> = None;
    let mut first: Option<Rc<RefCell<TreeNode>>> = None;
    let mut second: Option<Rc<RefCell<TreeNode>>> = None;

    traversal(root, &mut prev, &mut first, &mut second);
    if let (Some(first), Some(second)) = (first, second) {
        std::mem::swap(&mut first.borrow_mut().val, &mut second.borrow_mut().val);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vec_to_btree(data: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if data.len() == 0 {
            return None;
        }
        let bvtree: Vec<Option<Rc<RefCell<TreeNode>>>> = data
            .iter()
            .map(|&v| {
                if v == 999 {
                    None
                } else {
                    Some(Rc::new(RefCell::new(TreeNode::new(v))))
                }
            })
            .collect();
        let len = data.len();
        for i in 0..len / 2 {
            if let Some(node) = bvtree[i].clone() {
                let i1 = 2 * i + 1;
                let i2 = 2 * i + 2;
                node.borrow_mut().left = bvtree[i1].clone();
                node.borrow_mut().right = bvtree[i2].clone();
            }
        }
        bvtree[0].clone()
    }

    fn tf(input: Vec<i32>, expected: Vec<i32>) {
        let mut btree = vec_to_btree(&input);
        recover_tree(&mut btree);
        let expected = vec_to_btree(&expected);
        assert_eq!(btree, expected, "{:?}", input);
    }

    #[test]
    fn example() {
        tf(
            vec![1, 3, 999, 999, 2, 999, 999],
            vec![3, 1, 999, 999, 2, 999, 999],
        );

        tf(vec![5, 4, 6, 999, 999, 3, 7], vec![3, 4, 6, 999, 999, 5, 7]);
        tf(
            vec![3, 1, 4, 999, 999, 2, 999],
            vec![2, 1, 4, 999, 999, 3, 999],
        );
    }
}
