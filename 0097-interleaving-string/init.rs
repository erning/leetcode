pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
    let s1 = s1.as_bytes();
    let s2 = s2.as_bytes();
    let s3 = s3.as_bytes();
    let m = s1.len();
    let n = s2.len();
    if m + n != s3.len() {
        return false;
    }

    let mut dp: Vec<Vec<bool>> = vec![vec![false; m + 1]; n + 1];
    dp[0][0] = true;

    for (i, &v) in s1.iter().enumerate() {
        dp[0][i + 1] = v == s3[i] && dp[0][i];
    }
    for (i, &v) in s2.iter().enumerate() {
        dp[i + 1][0] = v == s3[i] && dp[i][0];
    }
    for (y, &b) in s2.iter().enumerate() {
        for (x, &a) in s1.iter().enumerate() {
            let c = s3[x + y + 1];
            if (a == c && dp[y + 1][x]) || (b == c && dp[y][x + 1]) {
                dp[y + 1][x + 1] = true;
            }
        }
    }

    dp[n][m]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(s1: &str, s2: &str, s3: &str, expected: bool) {
        assert_eq!(
            is_interleave(s1.to_string(), s2.to_string(), s3.to_string()),
            expected,
            "{:?}",
            (s1, s2, s3)
        );
    }

    #[test]
    fn example() {
        tf("aabcc", "dbbca", "aadbbcbcac", true);
        tf("aabcc", "dbbca", "aadbbbaccc", false);
        tf("", "", "", true);

        tf("a", "", "a", true);
        tf("", "a", "a", true);
        tf("db", "b", "cbb", false);
    }
}
