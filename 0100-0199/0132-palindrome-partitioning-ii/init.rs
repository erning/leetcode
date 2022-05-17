pub fn min_cut(s: String) -> i32 {
    fn is_palindrome(s: &str) -> bool {
        if s.len() <= 1 {
            return true;
        }
        let mut iter = s.chars();
        while let (Some(a), Some(b)) = (iter.next(), iter.next_back()) {
            if a != b {
                return false;
            }
        }
        true
    }

    let n = s.len();
    let mut dp: Vec<i32> = vec![n as i32; n + 1];
    dp[0] = -1;

    for i in 1..=s.len() {
        for j in 0..i {
            if is_palindrome(&s[j..i]) && dp[j] + 1 < dp[i] {
                dp[i] = dp[j] + 1;
            }
        }
    }

    dp[n]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(min_cut("aab".to_string()), 1);
        assert_eq!(min_cut("a".to_string()), 0);
        assert_eq!(min_cut("ab".to_string()), 1);

        assert_eq!(min_cut("aaa".to_string()), 0);
        assert_eq!(min_cut("abc".to_string()), 2);

        assert_eq!(min_cut("123456789".to_string()), 8);
        assert_eq!(min_cut("aabbabbacabb".to_string()), 2);
    }
}
