// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    head.as_ref()?;

    let mut len = 0;
    let mut curr = &head;
    while let Some(node) = curr {
        len += 1;
        curr = &node.next;
    }
    let k = len - (k % len);

    let mut head = head;
    let mut curr = &mut head;
    for _ in 0..k {
        if let Some(node) = curr {
            curr = &mut node.next;
        }
    }
    let mut new_head = curr.take();

    let mut curr = &mut new_head;
    while let Some(node) = curr {
        curr = &mut node.next;
    }
    curr.replace(head.unwrap());

    new_head
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

    fn tf(input: Vec<i32>, k: i32, expected: Vec<i32>) {
        let list = vec_to_list(&input);
        let list = rotate_right(list, k);
        let output = list_to_vec(&list);
        assert_eq!(output, expected);
    }

    #[test]
    fn example() {
        tf(vec![1, 2, 3, 4, 5], 2, vec![4, 5, 1, 2, 3]);
        tf(vec![0, 1, 2], 4, vec![2, 0, 1]);

        tf(vec![], 4, vec![]);
    }
}
