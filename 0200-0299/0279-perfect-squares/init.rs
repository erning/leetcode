pub fn num_squares(n: i32) -> i32 {
    let mut dp = vec![100_000; n as usize + 1];
    dp[0] = 0;
    let mut j = 1;
    for i in 1..=n {
        if j * j == i {
            dp[i as usize] = 1;
            j += 1;
            continue;
        }
        let mut k = 1;
        while k * k <= n {
            let a = k * k;
            let b = i - a;
            if a > b {
                break;
            }
            dp[i as usize] = i32::min(dp[i as usize], dp[a as usize] + dp[b as usize]);
            k += 1;
        }
    }
    *dp.last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(num_squares(12), 3);
        assert_eq!(num_squares(13), 2);
    }
}
