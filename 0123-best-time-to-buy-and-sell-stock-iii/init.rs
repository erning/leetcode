pub fn max_profit(prices: Vec<i32>) -> i32 {
    let len = prices.len();
    let mut b1 = vec![0; len];
    let mut s1 = vec![0; len];
    let mut b2 = vec![0; len];
    let mut s2 = vec![0; len];
    b1[0] = -prices[0];
    b2[0] = i32::MIN;
    for (i, p) in prices.into_iter().enumerate().skip(1) {
        b1[i] = i32::max(b1[i - 1], -p);
        s1[i] = i32::max(s1[i - 1], b1[i - 1] + p);
        b2[i] = i32::max(b2[i - 1], s1[i - 1] - p);
        s2[i] = i32::max(s2[i - 1], b2[i - 1] + p);
    }
    i32::max(s1[len - 1], s2[len - 1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(max_profit(vec![3, 3, 5, 0, 0, 3, 1, 4]), 6);
        assert_eq!(max_profit(vec![1, 2, 3, 4, 5]), 4);
        assert_eq!(max_profit(vec![7, 6, 4, 3, 1]), 0);

        assert_eq!(max_profit(vec![1]), 0);
        assert_eq!(max_profit(vec![1, 2]), 1);
        assert_eq!(max_profit(vec![1, 2, 3]), 2);
        assert_eq!(max_profit(vec![1, 2, 3, 4]), 3);
    }
}
