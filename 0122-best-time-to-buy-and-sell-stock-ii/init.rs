pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut b = vec![0; prices.len()];
    let mut s = vec![0; prices.len()];
    b[0] = -prices[0];
    for (i, p) in prices.into_iter().enumerate().skip(1) {
        b[i] = i32::max(s[i - 1] - p, b[i - 1]);
        s[i] = i32::max(b[i - 1] + p, s[i - 1]);
    }
    s[s.len() - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 7);
        assert_eq!(max_profit(vec![1, 2, 3, 4, 5]), 4);
        assert_eq!(max_profit(vec![7, 6, 4, 3, 1]), 0);
    }
}
