pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut dp = vec![1; n];
    let mut max = 1;
    for (i, a) in nums.iter().enumerate().skip(1) {
        let mut v = dp[i];
        for (j, b) in nums.iter().enumerate().take(i) {
            if a > b {
                let w = dp[j] + 1;
                if w > v {
                    v = w;
                }
            }
        }
        if v > max {
            max = v
        }
        dp[i] = v;
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! tf {
        ($input:expr, $expected:expr) => {
            let output = length_of_lis($input.to_vec());
            assert_eq!(output, $expected, "{:?}", $input);
        };
    }

    #[test]
    fn example() {
        tf!([10, 9, 2, 5, 3, 7, 101, 18], 4);
        tf!([0, 1, 0, 3, 2, 3], 4);
        tf!([7, 7, 7, 7, 7, 7, 7], 1);

        tf!([0, 1, 0, 3, 2, 3, -1, 0], 4);
    }
}
