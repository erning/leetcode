// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut prev: Option<Box<ListNode>> = None;
    let mut curr = head;
    while let Some(ref mut node) = curr {
        let next = std::mem::replace(&mut node.next, prev);
        prev = std::mem::replace(&mut curr, next);
    }

    curr = prev;
    prev = None;
    let mut i = 0;
    while let Some(ref mut node) = curr {
        i += 1;
        if i == n {
            let mut next = std::mem::take(&mut node.next);
            std::mem::swap(&mut curr, &mut next);
        } else {
            let next = std::mem::replace(&mut node.next, prev);
            prev = std::mem::replace(&mut curr, next);
        }
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

    fn tf(input: Vec<i32>, n: i32, expected: Vec<i32>) {
        let input_list = vec_to_list(&input);
        let output_list = remove_nth_from_end(input_list, n);
        let output = list_to_vec(&output_list);
        assert_eq!(output, expected);
    }

    #[test]
    fn example() {
        tf(vec![1, 2, 3, 4, 5], 2, vec![1, 2, 3, 5]);
        tf(vec![1], 1, vec![]);
        tf(vec![1, 2], 1, vec![1]);
    }
}
