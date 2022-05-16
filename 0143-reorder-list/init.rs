// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
    fn length(head: &Option<Box<ListNode>>) -> usize {
        let mut count = 0;
        let mut curr = head;
        while let Some(node) = curr {
            count += 1;
            curr = &node.next;
        }
        count
    }

    fn merge(a: Option<Box<ListNode>>, b: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev: Option<Box<ListNode>> = None;

        let mut curr_a: Option<Box<ListNode>> = a;
        let mut curr_b: Option<Box<ListNode>> = b;

        // while let (Some(mut node_a), Some(mut node_b)) = (curr_a, curr_b) {
        while let Some(mut node_b) = curr_b {
            if let Some(mut node_a) = curr_a {
                let next_a = node_a.next.take();
                let next_b = node_b.next.take();

                node_b.next = prev;
                node_a.next = Some(node_b);

                prev = Some(node_a);

                curr_a = next_a;
                curr_b = next_b;
            } else {
                node_b.next = prev;
                prev = Some(node_b);
                break;
            }
        }
        prev
    }

    let mut count = length(head);
    if count < 3 {
        return;
    }

    let mut prev: Option<Box<ListNode>> = None;
    let curr: &mut Option<Box<ListNode>> = head;

    while let Some(node) = curr {
        let next = std::mem::replace(&mut node.next, prev);
        prev = std::mem::replace(curr, next);
        count -= 2;
        if count < 2 {
            break;
        }
    }

    if count % 2 == 1 {
        if let Some(node) = curr {
            let next = std::mem::take(&mut node.next);
            let mut mid: Option<Box<ListNode>> = std::mem::replace(curr, next);
            mid.as_mut().unwrap().next = prev;
            prev = mid;
        }
    }

    let a = prev;
    let b = curr.take();
    let list = if count % 2 == 0 {
        merge(a, b)
    } else {
        merge(b, a)
    };

    *head = list;
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

    // #[test]
    // fn test_merge() {
    //     let a = vec![1, 2];
    //     let b = vec![3, 4];
    //     let list = merge(vec_to_list(&a), vec_to_list(&b));
    //     let c = list_to_vec(&list);
    //     assert_eq!(&c, &vec![2, 4, 1, 3]);
    // }

    fn tf(input: &[i32], expected: &[i32]) {
        let mut head = vec_to_list(input);
        reorder_list(&mut head);
        let output = list_to_vec(&head);
        assert_eq!(&output, expected);
    }

    #[test]
    fn example() {
        tf(&vec![1], &vec![1]);
        tf(&vec![1, 2], &vec![1, 2]);
        tf(&vec![1, 2, 3], &vec![1, 3, 2]);
        tf(&vec![1, 2, 3, 4], &vec![1, 4, 2, 3]);
        tf(&vec![1, 2, 3, 4, 5], &vec![1, 5, 2, 4, 3]);
    }
}
