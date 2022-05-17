pub fn jump(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    let mut dp: Vec<i32> = vec![i32::MAX; len];
    dp[0] = 0;
    let mut prev = 0;
    let mut skip = 1;
    for (i, &n) in nums.iter().take(len - 1).enumerate() {
        let curr = n as usize;
        let k = i + curr;
        if k >= len - 1 {
            return dp[i] + 1;
        }
        if skip < prev {
            skip = prev;
        }
        for j in i + skip..=k {
            dp[j] = i32::min(dp[j], dp[i] + 1);
        }
        prev = curr;
        if skip > 1 {
            skip -= 1;
        }
    }
    dp[len - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(jump(vec![2, 3, 1, 1, 4]), 2);
        assert_eq!(jump(vec![2, 3, 0, 1, 4]), 2);

        assert_eq!(
            jump(vec![
                8, 2, 4, 4, 4, 9, 5, 2, 5, 8, 8, 0, 8, 6, 9, 1, 1, 6, 3, 5, 1, 2, 6, 6, 0, 4, 8, 6,
                0, 3, 2, 8, 7, 6, 5, 1, 7, 0, 3, 4, 8, 3, 5, 9, 0, 4, 0, 1, 0, 5, 9, 2, 0, 7, 0, 2,
                1, 0, 8, 2, 5, 1, 2, 3, 9, 7, 4, 7, 0, 0, 1, 8, 5, 6, 7, 5, 1, 9, 9, 3, 5, 0, 7, 5
            ]),
            13,
        );
    }
}
