// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
    let mut a: Option<Box<ListNode>> = None;
    let mut b: Option<Box<ListNode>> = None;
    let mut ptr_a: &mut Option<Box<ListNode>> = &mut a;
    let mut ptr_b: &mut Option<Box<ListNode>> = &mut b;

    let mut curr = head;
    while let Some(mut node) = curr {
        let next = node.next.take();
        if node.val < x {
            ptr_a.replace(node);
            ptr_a = &mut ptr_a.as_mut().unwrap().next;
        } else {
            ptr_b.replace(node);
            ptr_b = &mut ptr_b.as_mut().unwrap().next;
        }
        curr = next;
    }

    *ptr_a = b;
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vec_to_list(data: &[i32]) -> Option<Box<ListNode>> {
        let mut node: Option<Box<ListNode>> = None;
        for v in data.iter().rev() {
            node = Some(Box::new(ListNode {
                next: node,
                val: *v,
            }));
        }
        node
    }

    fn list_to_vec(data: &Option<Box<ListNode>>) -> Vec<i32> {
        let mut rv: Vec<i32> = Vec::new();
        let mut data: &Option<Box<ListNode>> = data;
        while let Some(node) = data {
            rv.push(node.val);
            data = &node.next;
        }
        rv
    }

    fn tf(input: &[i32], x: i32, expected: &[i32]) {
        let input_list = vec_to_list(input);
        let output_list = partition(input_list, x);
        let output = list_to_vec(&output_list);
        assert_eq!(&output, expected);
    }

    #[test]
    fn example() {
        tf(&vec![1, 4, 3, 2, 5, 2], 3, &vec![1, 2, 2, 4, 3, 5]);
        tf(&vec![2, 1], 2, &vec![1, 2]);
        tf(&vec![], 1, &vec![]);
    }
}
