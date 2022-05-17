// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

use std::cell::RefCell;
use std::rc::Rc;

pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(node: Rc<RefCell<TreeNode>>, value: i32, sum: &mut i32) {
        let node = node.borrow();
        let value = value * 10 + node.val;
        match (node.left.as_ref(), node.right.as_ref()) {
            (Some(a), Some(b)) => {
                dfs(a.clone(), value, sum);
                dfs(b.clone(), value, sum);
            }
            (Some(a), None) => dfs(a.clone(), value, sum),
            (None, Some(b)) => dfs(b.clone(), value, sum),
            (None, None) => *sum += value,
        }
    }
    let mut sum = 0;
    if let Some(node) = root {
        dfs(node, 0, &mut sum);
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        sum_numbers(None);
    }
}
