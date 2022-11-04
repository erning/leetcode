pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    let mut dp: Vec<i32> = vec![amount + 1; amount as usize + 1];
    dp[0] = 0;
    for i in 0..=amount {
        for coin in coins.iter() {
            let j = i - coin;
            if j < 0 {
                continue;
            }
            let next = dp[j as usize] + 1;
            let vi = &mut dp[i as usize];
            if *vi > next {
                *vi = next
            }
        }
    }
    let count = dp[amount as usize];
    if count > amount {
        -1
    } else {
        count
    }
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
