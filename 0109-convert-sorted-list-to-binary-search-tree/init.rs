// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

use std::cell::RefCell;
use std::rc::Rc;

pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
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
    let mut nums: Vec<i32> = Vec::new();
    let mut curr = &head;
    while let Some(node) = curr.as_ref() {
        nums.push(node.val);
        curr = &node.next;
    }
    to_bst(&nums)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        sorted_list_to_bst(None);
    }
}
