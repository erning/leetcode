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

// fn recursive(output: &mut Vec<i32>, node_rr: Rc<RefCell<TreeNode>>) {
//     let node = node_rr.borrow_mut();
//     if let Some(child) = &node.left {
//         recursive(output, Rc::clone(child));
//     }
//     if let Some(child) = &node.right {
//         recursive(output, Rc::clone(child));
//     }
//     output.push(node.val);
// }

pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut output: Vec<i32> = Vec::new();
    if root.is_none() {
        return output;
    }
    // traverse(&mut output, root.unwrap());

    let mut stack: Vec<(Rc<RefCell<TreeNode>>, i32)> = Vec::new();
    stack.push((root.unwrap(), 0));
    while let Some((node_rr, mut step)) = stack.pop() {
        let node = node_rr.borrow();
        loop {
            match step {
                0 => {
                    if let Some(child) = &node.left {
                        stack.push((Rc::clone(&node_rr), step + 1));
                        stack.push((Rc::clone(child), 0));
                        break;
                    }
                }
                1 => {
                    if let Some(child) = &node.right {
                        stack.push((Rc::clone(&node_rr), step + 1));
                        stack.push((Rc::clone(child), 0));
                        break;
                    }
                }
                _ => {
                    output.push(node.val);
                    break;
                }
            }
            step += 1;
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
        let output = postorder_traversal(root);
        assert_eq!(output, expected);
    }

    #[test]
    fn example() {
        tf(vec![1, -1, 2, -1, -1, 3], vec![3, 2, 1]);
        tf(vec![], vec![]);
        tf(vec![1], vec![1]);
    }
}
