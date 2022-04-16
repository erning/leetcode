// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

use std::cell::RefCell;
use std::rc::Rc;

pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
    fn traversal(root: &mut Option<Rc<RefCell<TreeNode>>>, v: &mut Vec<Rc<RefCell<TreeNode>>>) {
        if let Some(node) = root {
            traversal(&mut node.borrow_mut().right.take(), v);
            traversal(&mut node.borrow_mut().left.take(), v);
            v.push(node.clone());
        }
    }

    let mut v: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
    traversal(root, &mut v);
    if let Some(mut prev) = v.pop() {
        while let Some(curr) = v.pop() {
            prev.borrow_mut().right.replace(curr.clone());
            prev = curr;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        flatten(&mut None);
    }
}
