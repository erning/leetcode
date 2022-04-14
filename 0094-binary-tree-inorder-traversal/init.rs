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

pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    fn traversal(rv: &mut Vec<i32>, node: Option<Rc<RefCell<TreeNode>>>) {
        if let Some(node) = node {
            let node = node.borrow();
            traversal(rv, node.left.clone());
            rv.push(node.val);
            traversal(rv, node.right.clone());
        }
    }

    let mut rv: Vec<i32> = Vec::new();
    traversal(&mut rv, root);
    rv
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
        let btree = vec_to_btree(&input);
        let output = inorder_traversal(btree);
        assert_eq!(output, expected, "{:?}", input);
    }

    #[test]
    fn example() {
        tf(vec![1, 999, 2, 999, 999, 3, 999], vec![1, 3, 2]);
        tf(vec![], vec![]);
        tf(vec![1], vec![1]);
    }
}
