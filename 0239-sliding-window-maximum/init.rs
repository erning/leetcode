use std::collections::VecDeque;

pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let mut answer = Vec::with_capacity(k);
    let mut mq = VecDeque::new(); // monotonic queue
    for v in nums.iter().take(k - 1) {
        while !mq.is_empty() && mq.back().unwrap() < v {
            mq.pop_back();
        }
        mq.push_back(*v);
    }
    for i in k - 1..nums.len() {
        let added = nums[i];
        while !mq.is_empty() && mq.back().unwrap() < &added {
            mq.pop_back();
        }
        mq.push_back(added);
        let max = *mq.front().unwrap();
        answer.push(max);

        let removed = nums[i + 1 - k];
        if removed == max {
            mq.pop_front();
        }
    }
    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(input: &[i32], k: i32, expected: &[i32]) {
        let output = max_sliding_window(input.to_vec(), k);
        assert_eq!(output, expected, "{:?}", (input, k));
    }

    #[test]
    fn example() {
        tf(&[1, 3, -1, -3, 5, 3, 6, 7], 3, &[3, 3, 5, 5, 6, 7]);
        tf(&[1], 1, &[1]);

        tf(&[1, 3, 1, 2, 0, 5], 3, &[3, 3, 2, 5]);
    }
}
