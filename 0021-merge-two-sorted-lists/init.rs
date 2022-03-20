// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut head: Option<Box<ListNode>> = None;
    let mut curr: &mut Option<Box<ListNode>> = &mut head;

    let mut a = list1;
    let mut b = list2;
    loop {
        let (mut node_a, mut node_b) = match (&a, &b) {
            (None, None) => break,
            (None, _) => {
                *curr = b;
                break;
            }
            (_, None) => {
                *curr = a;
                break;
            }
            _ => (a.unwrap(), b.unwrap()),
        };

        if node_a.val <= node_b.val {
            let next_a = node_a.next.take();
            curr.replace(node_a);
            curr = &mut curr.as_mut().unwrap().next;
            a = next_a;
            b = Some(node_b);
        } else {
            let next_b = node_b.next.take();
            curr.replace(node_b);
            curr = &mut curr.as_mut().unwrap().next;
            a = Some(node_a);
            b = next_b;
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

    #[test]
    fn vec_list_converter() {
        let a = vec![1, 2, 3, 4];
        let b = vec_to_list(&a);
        let c = list_to_vec(&b);
        assert_eq!(&a, &c);
    }

    fn tf(input1: &[i32], input2: &[i32], expected: &[i32]) {
        let list1 = vec_to_list(input1);
        let list2 = vec_to_list(input2);
        let output_list = merge_two_lists(list1, list2);
        let output = list_to_vec(&output_list);
        assert_eq!(&output, expected);
    }

    #[test]
    fn example() {
        tf(&vec![1, 2, 4], &vec![1, 3, 4], &vec![1, 1, 2, 3, 4, 4]);
        tf(&vec![], &vec![], &vec![]);
        tf(&vec![], &vec![0], &vec![0]);
    }
}
