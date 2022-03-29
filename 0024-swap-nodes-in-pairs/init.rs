#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
	if head.is_none() || head.as_ref().unwrap().next.is_none() {
		return head
	}

    let mut a = head;
    let mut b = std::mem::take(&mut a.as_mut().unwrap().next);
    let b_next = std::mem::take(&mut b.as_mut().unwrap().next);
    a.as_mut().unwrap().next = swap_pairs(b_next);
    b.as_mut().unwrap().next = a;

    b

	// a, b := head, head.Next
	// a.Next, b.Next = swapPairs(b.Next), a
	// return b
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
        let list = vec_to_list(input);
        let output_list = swap_pairs(list);
        let output = list_to_vec(&output_list);
        assert_eq!(&output, expected);
    }

    #[test]
    fn example() {
        tf(&vec![1, 2, 3, 4], &vec![2, 1, 4, 3]);
        tf(&vec![], &vec![]);
        tf(&vec![1], &vec![1]);
    }
}
