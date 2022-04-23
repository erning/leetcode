// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_none() {
        return head;
    }
    let mut list: Vec<Box<ListNode>> = Vec::new();
    let mut curr = head;
    while let Some(mut node) = curr {
        let next = node.next.take();
        list.push(node);
        curr = next;
    }
    list.sort_unstable_by_key(|v| v.val);
    let mut next = list.pop().unwrap();
    while let Some(mut curr) = list.pop() {
        curr.next.replace(next);
        next = curr;
    }
    Some(next)
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

    fn tf(input: &[i32], expected: &[i32]) {
        let input_list = vec_to_list(input);
        let output_list = sort_list(input_list);
        let output = list_to_vec(&output_list);
        assert_eq!(&output, expected, "{:?}", input);
    }

    #[test]
    fn example() {
        tf(&[4, 2, 1, 3], &[1, 2, 3, 4]);
        tf(&[-1, 5, 3, 4, 0], &[-1, 0, 3, 4, 5]);
    }
}
