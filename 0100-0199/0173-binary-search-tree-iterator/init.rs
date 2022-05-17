use std::cell::RefCell;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

pub struct BSTIterator {
    stack: Vec<(Rc<RefCell<TreeNode>>, bool)>,
}

impl BSTIterator {
    pub fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut stack = Vec::new();
        if let Some(node) = root {
            stack.push((node, false));
        }
        // if root.is_some() {
        //     stack.push((root.unwrap(), false));
        // }
        BSTIterator { stack }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> i32 {
        assert!(!self.stack.is_empty());
        while let Some((node, me)) = self.stack.pop() {
            if !me {
                self.stack.push((node.clone(), true));
                if let Some(next) = node.borrow().left.clone() {
                    self.stack.push((next, false));
                }
            } else {
                if let Some(next) = node.borrow().right.clone() {
                    self.stack.push((next, false));
                }
                return node.borrow().val;
            }
        }
        unreachable!()
    }

    pub fn has_next(&self) -> bool {
        !self.stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: None,
            right: None,
        })));
        let mut obj = BSTIterator::new(root);
        assert_eq!(obj.has_next(), true);
        assert_eq!(obj.next(), 0);
        assert_eq!(obj.has_next(), false);
    }
}
