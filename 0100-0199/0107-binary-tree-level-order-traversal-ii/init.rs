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

pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    fn get_children(
        nodes: Vec<Rc<RefCell<TreeNode>>>,
        rv: &mut Vec<Vec<i32>>,
    ) -> Vec<Rc<RefCell<TreeNode>>> {
        let mut children = Vec::new();
        let mut v = Vec::new();
        for node in nodes.into_iter() {
            let node = node.borrow();
            v.push(node.val);
            if let Some(node) = node.left.clone() {
                children.push(node);
            }
            if let Some(node) = node.right.clone() {
                children.push(node);
            }
        }
        rv.push(v);
        children
    }

    let mut rv = Vec::new();
    if let Some(root) = root {
        let mut nodes: Vec<Rc<RefCell<TreeNode>>> = vec![root];
        while !nodes.is_empty() {
            nodes = get_children(nodes, &mut rv);
        }
    }
    rv.reverse();
    rv
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(level_order_bottom(None), Vec::<Vec<i32>>::new());
    }
}
