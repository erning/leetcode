pub fn is_match(s: String, p: String) -> bool {
    let len_s = s.len();
    let len_p = p.len();

    let mut dp: Vec<Vec<bool>> = vec![vec![false; len_s + 1]; len_p + 1];
    dp[0][0] = true;
    let mut prev_p = b'*';

    for (i, ch_p) in p.as_bytes().iter().enumerate() {
        if *ch_p == b'*' {
            dp[i + 1][0] = dp[i - 1][0];
        }
        for (j, ch_s) in s.as_bytes().iter().enumerate() {
            dp[i + 1][j + 1] = match ch_p {
                b'*' => {
                    dp[i - 1][j + 1]
                        || (dp[i][j] || dp[i + 1][j]) && (prev_p == b'.' || prev_p == *ch_s)
                }
                b'.' => dp[i][j],
                _ => dp[i][j] && ch_p == ch_s,
            };
        }
        prev_p = *ch_p;
    }

    dp[len_p][len_s]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(is_match("aa".to_owned(), "a".to_owned()), false);
        assert_eq!(is_match("aa".to_owned(), "a*".to_owned()), true);
        assert_eq!(is_match("ab".to_owned(), ".*".to_owned()), true);

        assert_eq!(is_match("aab".to_owned(), "c*a*b".to_owned()), true);
        assert_eq!(is_match("aaa".to_owned(), "ab*a".to_owned()), false);
        assert_eq!(
            is_match("mississippi".to_owned(), "mis*is*p*.".to_owned()),
            false
        );
        assert_eq!(is_match("aaa".to_owned(), "ab*ac*a".to_owned()), true);
        assert_eq!(is_match("a".to_owned(), "ab*a".to_owned()), false);

        assert_eq!(is_match("aaab".to_owned(), "a*b".to_owned()), true);
        assert_eq!(is_match("aabb".to_owned(), "a*b".to_owned()), false);
        assert_eq!(is_match("babb".to_owned(), "a*b".to_owned()), false);
    }
}
