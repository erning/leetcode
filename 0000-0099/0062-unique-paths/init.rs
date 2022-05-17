pub fn unique_paths(m: i32, n: i32) -> i32 {
    let m = m as usize;
    let n = n as usize;
    let mut dp = vec![0; n + 1];
    dp[1] = 1;
    for _ in 0..m {
        for x in 0..n {
            dp[x + 1] += dp[x];
        }
    }
    dp[n]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(unique_paths(3, 7), 28);
        assert_eq!(unique_paths(3, 2), 3);
    }
}
