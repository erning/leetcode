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
use std::collections::VecDeque;
use std::rc::Rc;

pub struct Codec {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Default for Codec {
    fn default() -> Self {
        Self::new()
    }
}

impl Codec {
    pub fn new() -> Self {
        Codec {}
    }

    pub fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        if root.is_none() {
            return String::new();
        }
        let mut answer: Vec<Option<i32>> = Vec::new();

        let mut queue: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();
        queue.push_back(root);
        while let Some(node) = queue.pop_front() {
            if let Some(node) = node {
                answer.push(Some(node.borrow().val));
                queue.push_back(node.borrow().left.clone());
                queue.push_back(node.borrow().right.clone());
            } else {
                answer.push(None);
            }
        }

        while let Some(&v) = answer.last() {
            if v.is_some() {
                break;
            }
            answer.pop();
        }

        let mut rv = String::new();
        for v in answer {
            if let Some(v) = v {
                rv.push_str(v.to_string().as_str());
            } else {
                rv.push_str("null");
            }
            rv.push(',');
        }
        rv.pop();

        rv
    }

    pub fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let nodes: VecDeque<Option<Rc<RefCell<TreeNode>>>> = data
            .split(',')
            .map(|v| v.parse::<i32>().unwrap_or(i32::MAX))
            .map(|v| {
                if v != i32::MAX {
                    Some(Rc::new(RefCell::new(TreeNode::new(v))))
                } else {
                    None
                }
            })
            .collect();
        if nodes.is_empty() {
            return None;
        }

        let mut i = 0;
        let mut j = 0;
        while i < nodes.len() {
            if let Some(p) = nodes[i].clone() {
                j += 1;
                if j >= nodes.len() {
                    break;
                }
                p.borrow_mut().left = nodes[j].clone();
                j += 1;
                if j >= nodes.len() {
                    break;
                }
                p.borrow_mut().right = nodes[j].clone();
            }
            i += 1
        }

        nodes[0].clone()
    }
}

/**
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();>
 * let data: String = obj.serialize(strs);
 * let ans: Option<Rc<RefCell<TreeNode>>> = obj.deserialize(data);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let mut n1 = TreeNode::new(1);
        let n2 = TreeNode::new(2);
        let mut n3 = TreeNode::new(3);
        let n4 = TreeNode::new(4);
        let n5 = TreeNode::new(5);
        n3.left = Some(Rc::new(RefCell::new(n4)));
        n3.right = Some(Rc::new(RefCell::new(n5)));
        n1.left = Some(Rc::new(RefCell::new(n2)));
        n1.right = Some(Rc::new(RefCell::new(n3)));
        let root = Some(Rc::new(RefCell::new(n1)));

        let obj = Codec::new();
        let data = obj.serialize(root);
        // println!("data={:?}", data);
        let tree = obj.deserialize(data);
        // println!("tree={:?}", tree);
    }
}
