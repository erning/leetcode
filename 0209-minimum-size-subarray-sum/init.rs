pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut sum = 0;
    for &v in nums.iter() {
        if v >= target {
            return 1;
        }
        sum += v;
    }
    if sum < target {
        return 0;
    }
    if sum == target {
        return n as i32;
    }
    let mut dp = nums.clone();
    for l in 1..n {
        for i in (l..n).rev() {
            dp[i] = dp[i - 1] + nums[i];
            if dp[i] >= target {
                return (l + 1) as i32;
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(min_sub_array_len(3, vec![1, 1, 1]), 3);
        assert_eq!(min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]), 2);
        assert_eq!(min_sub_array_len(4, vec![1, 4, 4]), 1);
        assert_eq!(min_sub_array_len(11, vec![1, 1, 1, 1, 1, 1, 1, 1]), 0);

        assert_eq!(min_sub_array_len(15, vec![2, 14]), 2);
    }
}
