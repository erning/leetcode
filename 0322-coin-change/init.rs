pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    let f = dp;
    match f(&coins, amount) {
        Some(count) => count,
        None => -1,
    }
}

fn dp(coins: &[i32], amount: i32) -> Option<i32> {
    let mut dp: Vec<Option<i32>> = vec![None; amount as usize + 1];
    dp[0].replace(0);

    for i in 0..=amount {
        for coin in coins.iter() {
            let j = i - coin;
            if j < 0 {
                continue;
            }
            let i = i as usize;
            let j = j as usize;
            if let Some(prev) = dp[j] {
                let next = prev + 1;
                if let Some(count) = dp[i] {
                    if count > next {
                        dp[i].replace(next);
                    }
                } else {
                    dp[i].replace(next);
                }
            }
        }
    }
    dp[amount as usize]
}

#[allow(dead_code)]
fn recursion(coins: &[i32], amount: i32) -> Option<i32> {
    if amount < 0 {
        return None;
    }
    if amount == 0 {
        return Some(0);
    }
    let mut rv = None;
    for coin in coins.iter() {
        if let Some(prev) = recursion(coins, amount - *coin) {
            let next = prev + 1;
            if let Some(count) = rv {
                if count > next {
                    rv.replace(next);
                }
            } else {
                rv.replace(next);
            }
        }
    }
    rv
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(coin_change(vec![1, 2, 5], 11), 3);
        assert_eq!(coin_change(vec![2], 3), -1);
        assert_eq!(coin_change(vec![1], 0), 0);
    }
}
