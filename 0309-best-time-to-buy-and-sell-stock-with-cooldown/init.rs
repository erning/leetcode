pub fn max_profit(prices: Vec<i32>) -> i32 {
    let (a, _, b) = prices
        .iter()
        .fold((-prices[0], 0, 0), |(a2, b1, b2), price| {
            (i32::max(a2, b1 - price), b2, i32::max(b2, a2 + price))
        });
    i32::max(a, b)

    // let len = prices.len();
    // let mut dp: Vec<Vec<i32>> = vec![vec![0; len + 1]; 2];
    // dp[0][1] = -prices[0];
    // for i in 1..len {
    //     dp[0][i + 1] = i32::max(dp[0][i], dp[1][i - 1] - prices[i]);
    //     dp[1][i + 1] = i32::max(dp[1][i], dp[0][i] + prices[i]);
    // }
    // i32::max(dp[0][len], dp[1][len])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(max_profit(vec![1, 2, 3, 0, 2]), 3);
        assert_eq!(max_profit(vec![1]), 0);
    }
}
