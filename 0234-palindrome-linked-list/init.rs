// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
    fn length(head: &Option<Box<ListNode>>) -> usize {
        let mut count = 0;
        let mut curr = head;
        while let Some(node) = curr {
            count += 1;
            curr = &node.next;
        }
        count
    }
    let mut count = length(&head);
    if count <= 1 {
        return true;
    }

    let mut prev: Option<Box<ListNode>> = None;
    let mut curr = head;
    while let Some(mut node) = curr {
        let next = std::mem::replace(&mut node.next, prev);
        prev = Some(node);
        curr = next;
        count -= 2;
        if count < 2 {
            break;
        }
    }
    if count % 2 == 1 {
        curr = curr.unwrap().next
    }

    while let (Some(a), Some(b)) = (prev, curr) {
        if a.val != b.val {
            return false;
        }
        prev = a.next;
        curr = b.next;
    }
    true
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

    fn tf(input: &[i32], expected: bool) {
        let list = vec_to_list(input);
        let output = is_palindrome(list);
        assert_eq!(output, expected);
    }

    #[test]
    fn example() {
        tf(&vec![1, 2, 3, 2, 1], true);
        tf(&vec![1, 2, 2, 1], true);
        tf(&vec![1, 2], false);
        tf(&vec![1], true);
    }
}
