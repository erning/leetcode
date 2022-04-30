pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
    let n = prices.len();
    if n < 2 {
        return 0;
    }

    let m = usize::min(n, (k * 2) as usize);
    let mut dp: Vec<Vec<i32>> = vec![vec![0; n]; m + 1];

    dp[0][0] = -prices[0];
    for (i, &p) in prices.iter().enumerate().skip(1) {
        dp[0][i] = i32::max(dp[0][i - 1], -p);
    }

    let mut max = 0;
    for j in 1..=m {
        let sign = if j % 2 == 0 { -1 } else { 1 };
        dp[j][j - 1] = i32::MIN;
        for (i, &p) in prices.iter().enumerate().skip(j) {
            let v = i32::max(dp[j][i - 1], dp[j - 1][i - 1] + sign * p);
            dp[j][i] = v;
            if v > max {
                max = v
            }
        }
    }

    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(max_profit(2, vec![2, 4, 1]), 2);
        assert_eq!(max_profit(2, vec![3, 2, 6, 5, 0, 3]), 7);

        assert_eq!(max_profit(2, vec![1, 2, 4]), 3);
        assert_eq!(max_profit(2, vec![1, 2, 4, 7]), 6);

        assert_eq!(max_profit(2, vec![]), 0);
        assert_eq!(max_profit(2, vec![1]), 0);
        assert_eq!(max_profit(2, vec![1, 2]), 1);
    }
}
