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
        a: i32,
        b: i32,
        lca: &mut Option<Rc<RefCell<TreeNode>>>,
    ) {
        if lca.is_some() {
            return;
        }
        if let Some(ref node) = root {
            let val = node.borrow().val;
            if a <= val && b >= val {
                *lca = root;
                return;
            }
            let next = if b <= val {
                node.borrow().left.clone()
            } else if a >= val {
                node.borrow().right.clone()
            } else {
                return;
            };
            traverse(next, a, b, lca);
        }
    }

    let mut lca: Option<Rc<RefCell<TreeNode>>> = None;
    let mut a = p.unwrap().borrow().val;
    let mut b = q.unwrap().borrow().val;
    if a > b {
        std::mem::swap(&mut a, &mut b);
    }
    traverse(root, a, b, &mut lca);
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
