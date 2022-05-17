// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

use std::cell::RefCell;
use std::rc::Rc;

pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>, max: &mut i32) -> i32 {
        if let Some(node) = root {
            let v = node.borrow().val;
            let a = max_path_sum(node.borrow().left.clone(), max);
            let b = max_path_sum(node.borrow().right.clone(), max);
            let s = v + if a > 0 { a } else { 0 } + if b > 0 { b } else { 0 };
            if s > *max {
                *max = s;
            }
            i32::max(v, i32::max(a, b) + v)
        } else {
            0
        }
    }
    let mut m = i32::MIN;
    let n = max_path_sum(root, &mut m);
    i32::max(m, n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        max_path_sum(None);
    }
}
