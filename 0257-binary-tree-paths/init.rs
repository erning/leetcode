// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

use std::cell::RefCell;
use std::rc::Rc;

pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
    fn f(root: Option<Rc<RefCell<TreeNode>>>, prev_path: &Vec<i32>, paths: &mut Vec<String>) {
        if let Some(node) = root {
            let val = node.borrow().val;
            let mut path = prev_path.clone();
            path.push(val);
            let left = node.borrow().left.clone();
            let right = node.borrow().right.clone();
            if left.is_none() && right.is_none() {
                let mut s = path[0].to_string();
                for v in path.iter().skip(1) {
                    s.push_str("->");
                    s.push_str(v.to_string().as_str());
                }
                paths.push(s);
            } else {
                f(left, &path, paths);
                f(right, &path, paths);
            }
        }
    }
    let mut paths: Vec<String> = Vec::new();
    f(root, &Vec::new(), &mut paths);
    paths
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        binary_tree_paths(None);
    }
}
