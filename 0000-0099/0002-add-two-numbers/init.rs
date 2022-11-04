// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

#[allow(dead_code)]
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut l1 = l1;
    let mut l2 = l2;
    let mut stack: Vec<i32> = Vec::with_capacity(101);
    let mut c = 0;
    while l1.is_some() || l2.is_some() {
        let a = match l1 {
            Some(v) => {
                l1 = v.next;
                v.val
            }
            _ => 0,
        };
        let b = match l2 {
            Some(v) => {
                l2 = v.next;
                v.val
            }
            _ => 0,
        };
        let s = a + b + c;
        c = i32::from(s >= 10);
        stack.push(s % 10);
    }
    if c > 0 {
        stack.push(c)
    }
    let mut next: Option<Box<ListNode>> = None;
    while let Some(val) = stack.pop() {
        next = Some(Box::new(ListNode { val, next }))
    }
    next
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_list(v: i32) -> Option<Box<ListNode>> {
        let mut next: Option<Box<ListNode>> = None;
        let mut v = v;
        while v > 0 {
            let val = v % 10;
            v = v / 10;
            next = Some(Box::new(ListNode { val, next }));
        }
        next
    }

    fn tf(a: i32, b: i32, expected: i32) {
        let a = build_list(a);
        let b = build_list(b);
        let expected = build_list(expected);
        let output = add_two_numbers(a, b);
        assert_eq!(output, expected);
    }

    #[test]
    fn example() {
        tf(243, 564, 708);
        tf(9999999, 9999, 89990001);
    }
}
