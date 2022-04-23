// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

use std::cell::RefCell;
use std::rc::Rc;

pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
    fn path_sum(
        root: Rc<RefCell<TreeNode>>,
        path: &mut Vec<i32>,
        paths: &mut Vec<Vec<i32>>,
        target_sum: i32,
    ) {
        let node = root.borrow();
        path.push(node.val);
        let target = target_sum - node.val;
        match (node.left.clone(), node.right.clone()) {
            (Some(a), Some(b)) => {
                path_sum(a, path, paths, target);
                path_sum(b, path, paths, target);
            }
            (Some(a), None) => {
                path_sum(a, path, paths, target);
            }
            (None, Some(b)) => {
                path_sum(b, path, paths, target);
            }
            (None, None) => {
                if target == 0 {
                    paths.push(path.clone());
                }
            }
        }
        path.pop();
    }

    let mut paths: Vec<Vec<i32>> = vec![];
    let mut path: Vec<i32> = vec![];
    if let Some(node) = root {
        path_sum(node, &mut path, &mut paths, target_sum);
    }
    paths
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        path_sum(None, 0);
    }
}
