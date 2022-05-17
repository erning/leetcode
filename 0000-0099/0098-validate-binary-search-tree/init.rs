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

pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Option<(i32, i32)> {
        let root = root.as_ref().unwrap().borrow();
        let mut a = root.val;
        let mut b = root.val;

        if let Some(node) = root.left.as_ref() {
            if root.val <= node.borrow().val {
                return None;
            }
            if let Some((a1, b1)) = traversal(root.left.clone()) {
                if a <= b1 {
                    return None;
                }
                a = a1;
            } else {
                return None;
            }
        }

        if let Some(node) = root.right.as_ref() {
            if root.val >= node.borrow().val {
                return None;
            }
            if let Some((a1, b1)) = traversal(root.right.clone()) {
                if b >= a1 {
                    return None;
                }
                b = b1;
            } else {
                return None;
            }
        }

        Some((a, b))
    }

    traversal(root).is_some()
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

    fn tf(input: Vec<i32>, expected: bool) {
        let btree = vec_to_btree(&input);
        let output = is_valid_bst(btree);
        assert_eq!(output, expected, "{:?}", input);
    }

    #[test]
    fn example() {
        tf(vec![2, 1, 3], true);
        tf(vec![5, 1, 4, 999, 999, 3, 6], false);

        tf(vec![2, 2, 2], false);
        tf(vec![5, 4, 6, 999, 999, 3, 7], false);

        let mut tree: Vec<i32> = vec![999; 32 - 1];
        tree[0] = 3;
        tree[2] = 30;
        tree[5] = 10;
        tree[12] = 15;
        tree[26] = 45;
        tf(tree, false);
    }
}
