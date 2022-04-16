pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut a = prices[0];
    let mut max = 0;
    for b in prices.into_iter().skip(1) {
        let profit = b - a;
        if profit > max {
            max = profit
        }
        if b < a {
            a = b;
        }
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {

        assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
        assert_eq!(max_profit(vec![7, 6, 4, 3, 1]), 0);

        assert_eq!(max_profit(vec![3, 2, 6, 5, 0, 3]), 4);
        assert_eq!(max_profit(vec![1]), 0);
    }
}
