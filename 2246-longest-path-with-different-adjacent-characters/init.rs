use std::cell::RefCell;
use std::rc::Rc;

pub fn longest_path(parent: Vec<i32>, s: String) -> i32 {
    #[derive(Debug)]
    struct TreeNode {
        val: char,
        children: Vec<Rc<RefCell<TreeNode>>>,
    }

    impl TreeNode {
        fn new(val: char) -> TreeNode {
            TreeNode {
                val,
                children: Vec::new(),
            }
        }
    }

    fn max_path(parent: Rc<RefCell<TreeNode>>, max: &mut i32) -> i32 {
        let node = parent.borrow();
        let subpaths: Vec<i32> = node
            .children
            .iter()
            .map(|child| max_path(child.clone(), max))
            .collect();

        let mut submax = 0;
        for i in 0..subpaths.len() {
            if node.val == node.children[i].borrow().val {
                continue;
            }
            let a = subpaths[i];
            if a > submax {
                submax = a;
            }
            for j in i + 1..subpaths.len() {
                if node.val == node.children[j].borrow().val {
                    continue;
                }
                let b = subpaths[j];
                let s = a + b + 1;
                if s > *max {
                    *max = s;
                }
            }
        }
        let s = submax + 1;
        if s > *max {
            *max = s;
        }
        s
    }

    let nodes: Vec<Rc<RefCell<TreeNode>>> = s
        .chars()
        .into_iter()
        .map(|c| Rc::new(RefCell::new(TreeNode::new(c))))
        .collect();
    for (i, node) in nodes.iter().enumerate().skip(1).rev() {
        nodes[parent[i] as usize]
            .borrow_mut()
            .children
            .push(node.clone());
    }

    let mut max = 0;
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
