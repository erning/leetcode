use std::cell::RefCell;
use std::rc::Rc;

pub fn longest_path(parent: Vec<i32>, s: String) -> i32 {
    #[derive(Debug)]
    struct TreeNode {
        val: u8,
        children: Vec<Rc<RefCell<TreeNode>>>,
    }

    impl TreeNode {
        fn new(val: u8) -> TreeNode {
            TreeNode {
                val,
                children: Vec::new(),
            }
        }
    }

    fn max_path(parent: Rc<RefCell<TreeNode>>, max: &mut i32) -> i32 {
        let node = parent.borrow();
        let children: Vec<(u8, i32)> = node
            .children
            .iter()
            .map(|child| (child.borrow().val, max_path(child.clone(), max)))
            .collect();

        let mut smax = 0;
        for (i, &(_, a)) in children
            .iter()
            .enumerate()
            .filter(|(_, &(v, _))| node.val != v)
        {
            if a > smax {
                smax = a;
            }
            for &(_, b) in children.iter().skip(i + 1).filter(|&(v, _)| node.val != *v) {
                let s = a + b + 1;
                if s > *max {
                    *max = s;
                }
            }
        }
        let s = smax + 1;
        if s > *max {
            *max = s;
        }
        s
    }

    let nodes: Vec<Rc<RefCell<TreeNode>>> = s
        .as_bytes()
        .into_iter()
        .map(|&c| Rc::new(RefCell::new(TreeNode::new(c))))
        .collect();
    for (i, node) in nodes.iter().enumerate().skip(1).rev() {
        nodes[parent[i] as usize]
            .borrow_mut()
            .children
            .push(node.clone());
    }

    let mut max = 1;
    max_path(nodes[0].clone(), &mut max);
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            longest_path(vec![-1, 0, 0, 1, 1, 2], "abacbe".to_string()),
            3
        );
        assert_eq!(longest_path(vec![-1, 0, 0, 0], "aabc".to_string()), 3);
        assert_eq!(longest_path(vec![-1, 0, 1], "aab".to_string()), 2);
    }
}
