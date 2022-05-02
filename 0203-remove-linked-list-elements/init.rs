// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
    let mut head = head;
    let mut curr = &mut head;
    while curr.is_some() {
        if curr.as_ref().unwrap().val == val {
            *curr = curr.as_mut().unwrap().next.take();
        } else {
            curr = &mut curr.as_mut().unwrap().next;
        }
    }
    head
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

    fn tf(input: &[i32], val: i32, excepted: &[i32]) {
        let list = vec_to_list(input);
        let list = remove_elements(list, val);
        let output = list_to_vec(&list);
        assert_eq!(output, excepted);
    }

    #[test]
    fn example() {
        tf(&[1, 2, 6, 3, 4, 5, 6], 6, &[1, 2, 3, 4, 5]);
        tf(&[], 1, &[]);
        tf(&[7, 7, 7, 7], 7, &[]);
    }
}
