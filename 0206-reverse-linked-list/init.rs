// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev: Option<Box<ListNode>> = None;
    let mut curr = head;
    while let Some(ref mut node) = curr {
        // let next = std::mem::take(&mut node.next);
        // node.next = prev;
        // prev = curr;
        // curr = next;
        let next = std::mem::replace(&mut node.next, prev);
        prev = std::mem::replace(&mut curr, next);
    }
    prev
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

    #[test]
    fn vec_list_converter() {
        let a = vec![1, 2, 3, 4];
        let b = vec_to_list(&a);
        let c = list_to_vec(&b);
        assert_eq!(&a, &c);
    }

    fn tf(input: &[i32], expected: &[i32]) {
        let input_list = vec_to_list(input);
        let output_list = reverse_list(input_list);
        let output = list_to_vec(&output_list);
        assert_eq!(&output, expected);
    }

    #[test]
    fn example() {
        tf(&vec![], &vec![]);
        tf(&vec![1], &vec![1]);
        tf(&vec![1, 2], &vec![2, 1]);
        tf(&vec![1, 2, 3, 4], &vec![4, 3, 2, 1]);
        tf(&vec![1, 2, 3, 4, 5], &vec![5, 4, 3, 2, 1]);
    }
}
