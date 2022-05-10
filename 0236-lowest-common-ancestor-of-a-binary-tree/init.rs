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

pub fn lowest_common_ancestor(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    fn traverse(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: &mut Option<Rc<RefCell<TreeNode>>>,
        q: &mut Option<Rc<RefCell<TreeNode>>>,
        path: &mut Vec<Rc<RefCell<TreeNode>>>,
        path_p: &mut Vec<Rc<RefCell<TreeNode>>>,
        path_q: &mut Vec<Rc<RefCell<TreeNode>>>,
    ) {
        if p.is_none() && q.is_none() {
            return;
        }
        if let Some(ref node) = root {
            path.push(node.clone());
            if let Some(p_node) = p {
                if node == p_node {
                    *path_p = path.clone();
                    *p = None;
                }
            }
            if let Some(q_node) = q {
                if node == q_node {
                    *path_q = path.clone();
                    *q = None;
                }
            }
            let left = node.borrow().left.clone();
            let right = node.borrow().right.clone();
            traverse(left, p, q, path, path_p, path_q);
            traverse(right, p, q, path, path_p, path_q);
            path.pop();
        }
    }

    let mut p = p;
    let mut q = q;
    let mut path: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
    let mut path_p: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
    let mut path_q: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
    traverse(root, &mut p, &mut q, &mut path, &mut path_p, &mut path_q);

    let mut lca: Option<Rc<RefCell<TreeNode>>> = None;
    let mut i = 0;
    while i < path_p.len() && i < path_q.len() {
        if path_p[i] != path_q[i] {
            break;
        }
        lca.replace(path_p[i].clone());
        i += 1;
    }
    lca
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let a = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let b = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: a.clone(),
            right: b.clone(),
        })));
        lowest_common_ancestor(root, a, b);
    }
}
