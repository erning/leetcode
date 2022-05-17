#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

use std::cell::RefCell;
use std::rc::Rc;

pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let lh = {
        let mut h = 0;
        let mut v = root.clone();
        while let Some(node) = v {
            h += 1;
            v = node.borrow().left.clone();
        }
        h
    };
    let rh = {
        let mut h = 0;
        let mut v = root.clone();
        while let Some(node) = v {
            h += 1;
            v = node.borrow().right.clone();
        }
        h
    };
    if lh == rh {
        (1 << lh) - 1
    } else if let Some(node) = root {
        let a = count_nodes(node.borrow().left.clone());
        let b = count_nodes(node.borrow().right.clone());
        a + b + 1
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        count_nodes(None);
    }
}
