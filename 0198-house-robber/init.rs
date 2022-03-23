pub fn rob(nums: Vec<i32>) -> i32 {
    if nums.len() == 1 {
        return nums[0];
    }
    let mut dp: Vec<i32> = vec![0; nums.len()];
    dp[0] = nums[0];
    dp[1] = i32::max(nums[1], dp[0]);

    for i in 2..dp.len() {
        dp[i] = i32::max(nums[i] + dp[i - 2], dp[i - 1]);
    }

    dp[dp.len() - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(rob(vec![1,2,3,1]), 4);
        assert_eq!(rob(vec![2,7,9,3,1]), 12);

        assert_eq!(rob(vec![2,1]), 2);
    }
}
