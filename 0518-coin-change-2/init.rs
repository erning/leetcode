pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
    if amount <= 0 {
        return 1;
    }
    let mut dp: Vec<i32> = vec![0; amount as usize + 1];
    dp[0] = 1;
    for coin in coins.iter() {
        for i in 1..=amount {
            let j = i - coin;
            if j >= 0 {
                dp[i as usize] += dp[j as usize];
            }
        }
    }
    dp[amount as usize]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(change(5, vec![1, 2, 5]), 4);
        assert_eq!(change(3, vec![2]), 0);
        assert_eq!(change(10, vec![10]), 1);
    }
}
