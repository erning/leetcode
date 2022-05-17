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

pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    fn to_bst(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.is_empty() {
            return None;
        }
        let m = nums.len() / 2;
        let node = TreeNode {
            val: nums[m],
            left: to_bst(&nums[..m]),
            right: to_bst(&nums[m + 1..]),
        };
        Some(Rc::new(RefCell::new(node)))
    }
    to_bst(&nums)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_ne!(sorted_array_to_bst(vec![-10, -3, 0, 5, 9]), None);
        assert_ne!(sorted_array_to_bst(vec![1, 3]), None);
    }
}
