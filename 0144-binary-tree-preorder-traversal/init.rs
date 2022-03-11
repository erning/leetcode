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
pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut output: Vec<i32> = Vec::new();
    if root.is_none() {
        return output;
    }
    let mut stack: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
    stack.push(root.unwrap());
    while let Some(node) = stack.pop() {
        let node = node.borrow_mut();
        output.push(node.val);
        if let Some(child) = &node.right {
            stack.push(Rc::clone(child));
        }
        if let Some(child) = &node.left {
            stack.push(Rc::clone(child));
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    fn build_tree(input: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        if input.is_empty() || input.first().is_none() {
            return None;
        }
        let root = Rc::new(RefCell::new(TreeNode {
            val: input[0].unwrap(),
            left: None,
            right: None,
        }));
        let mut output: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![None; input.len()];
        output[0] = Some(Rc::clone(&root));

        for (i, v) in input.iter().enumerate().skip(1) {
            output[i] = match v {
                Some(v) => {
                    let node: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode {
                        val: *v,
                        left: None,
                        right: None,
                    }));
                    if i > 0 {
                        let p = (i + 1) / 2 - 1;
                        match &output[p] {
                            Some(parent) => {
                                if i % 2 == 1 {
                                    parent.borrow_mut().left = Some(Rc::clone(&node));
                                } else {
                                    parent.borrow_mut().right = Some(Rc::clone(&node));
                                }
                            }
                            None => {}
                        }
                    }
                    Some(Rc::clone(&node))
                }
                None => None,
            };
        }

        Some(root)
    }

    fn tf(input: Vec<i32>, expected: Vec<i32>) {
        let input = input
            .iter()
            .map(|v| match v {
                -1 => None,
                _ => Some(*v),
            })
            .collect();
        let root = build_tree(input);
        let output = preorder_traversal(root);
        assert_eq!(output, expected);
    }

    #[test]
    fn example() {
        tf(vec![1, -1, 2, -1, -1, 3], vec![1, 2, 3]);
        tf(vec![], vec![]);
        tf(vec![1], vec![1]);
    }
}
