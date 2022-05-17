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

pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let mut stack_a: Vec<Option<Rc<RefCell<TreeNode>>>> = Vec::new();
    let mut stack_b: Vec<Option<Rc<RefCell<TreeNode>>>> = Vec::new();
    let root = root.unwrap();
    let root = root.borrow();
    stack_a.push(root.left.clone());
    stack_b.push(root.right.clone());

    while let (Some(a), Some(b)) = (stack_a.pop(), stack_b.pop()) {
        if a.is_none() && b.is_some() {
            return false;
        }
        if a.is_some() && b.is_none() {
            return false;
        }
        if a.is_none() && b.is_none() {
            continue;
        }
        let a = a.unwrap();
        let b = b.unwrap();
        let a = a.borrow();
        let b = b.borrow();
        if a.val != b.val {
            return false;
        }

        stack_a.push(a.right.clone());
        stack_a.push(a.left.clone());

        stack_b.push(b.left.clone());
        stack_b.push(b.right.clone());
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vec_to_btree(data: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
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

    fn tf(input: &[i32], expected: bool) {
        let list = vec_to_btree(input);
        let output = is_symmetric(list);
        assert_eq!(output, expected);
    }

    #[test]
    fn example() {
        tf(&vec![1, 2, 2, 3, 4, 4, 3], true);
        tf(&vec![1, 2, 2, 999, 3, 999, 3], false);
        tf(&vec![1], true);
    }
}
