use crate::Solution;

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

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut a1 = l1.unwrap();
        let mut a2 = l2.unwrap();
        let mut rv: Vec<i32> = Vec::new();
        let mut v1 = a1.val;
        let mut v2 = a2.val;
        let mut c = 0;
        loop {
            let mut done = 0;
            let sum = c + v1 + v2;
            c = sum / 10;
            rv.push(sum % 10);
            match a1.next {
                Some(v) => {
                    a1 = v;
                    v1 = a1.val;
                }
                _ => {
                    v1 = 0;
                    done += 1;
                }
            }
            match a2.next {
                Some(v) => {
                    a2 = v;
                    v2 = a2.val;
                }
                _ => {
                    v2 = 0;
                    done += 1;
                }
            }
            if done >= 2 && c == 0 {
                break;
            }
        }
        let mut prev: Option<Box<ListNode>> = None;
        for v in rv.iter().rev() {
            prev = Some(Box::new(ListNode {
                val: *v,
                next: prev,
            }));
        }
        prev
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
        let mut prev: Option<Box<ListNode>> = None;
        for v in vec.iter().rev() {
            prev = Some(Box::new(ListNode {
                val: *v,
                next: prev,
            }));
        }
        prev
    }

    fn tf(a: Vec<i32>, b: Vec<i32>, expected: Vec<i32>) {
        let a = build_list(a);
        let b = build_list(b);
        let expected = build_list(expected);
        let output = Solution::add_two_numbers(a, b);
        assert_eq!(output, expected);
    }

    #[test]
    fn example() {
        // let nums: Vec<i32> = vec![2, 7, 11, 15];
        // let target: i32 = 9;
        // let expected: Vec<i32> = vec![0, 1];
        // let output = Solution::two_sum(nums, target);

        // assert_eq!(expected, output);
        tf(vec![2, 4, 3], vec![5, 6, 4], vec![7, 0, 8]);
        tf(
            vec![9, 9, 9, 9, 9, 9, 9],
            vec![9, 9, 9, 9],
            vec![8, 9, 9, 9, 0, 0, 0, 1],
        );
    }
}
