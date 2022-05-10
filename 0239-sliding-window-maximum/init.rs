pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let mut answer = Vec::with_capacity(k);
    let mut max = *(&nums[..k]).iter().max().unwrap();
    let mut removed = nums[0];
    answer.push(max);
    for v in nums.windows(k).skip(1) {
        let added = v[k - 1];
        if added > max {
            max = added;
        } else if removed == max && added < max {
            max = *v.iter().max().unwrap();
        }
        removed = v[0];
        answer.push(max);
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
    }
}
