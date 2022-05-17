// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut num = i32::MAX;

    let mut new: Option<Box<ListNode>> = None;
    let mut ptr: &mut Option<Box<ListNode>> = &mut new;

    let mut curr = head;
    while let Some(mut node) = curr {
        let next = node.next.take();
        if num != node.val {
            num = node.val;
            ptr.replace(node);
            ptr = &mut ptr.as_mut().unwrap().next;
        }
        curr = next;
    }
    new
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
        let output_list = delete_duplicates(input_list);
        let output = list_to_vec(&output_list);
        assert_eq!(&output, expected);
    }

    #[test]
    fn example() {
        tf(&vec![1, 1, 2], &vec![1, 2]);
        tf(&vec![1, 1, 2, 3, 3], &vec![1, 2, 3]);
        tf(&vec![], &vec![]);
    }
}
