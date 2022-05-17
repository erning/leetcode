pub fn min_distance(word1: String, word2: String) -> i32 {
    let len1 = word1.len();
    let len2 = word2.len();
    if len1 == 0 {
        return len2 as i32;
    }
    if len2 == 0 {
        return len1 as i32;
    }

    let word1 = word1.as_bytes();
    let word2 = word2.as_bytes();
    let mut dp = vec![vec![0; len2 + 1]; len1 + 1];

    #[allow(clippy::needless_range_loop)]
    for i in 0..=len1 {
        dp[i][0] = i;
    }
    for j in 0..=len2 {
        dp[0][j] = j;
    }
    for i in 1..=len1 {
        for j in 1..=len2 {
            let mut v = dp[i - 1][j - 1];
            if word1[i - 1] != word2[j - 1] {
                let b = dp[i - 1][j];
                let c = dp[i][j - 1];
                if b < v {
                    v = b;
                }
                if c < v {
                    v = c;
                }
                v += 1
            }
            dp[i][j] = v;
        }
    }

    dp[len1][len2] as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(min_distance("horse".to_string(), "ros".to_string()), 3);
        assert_eq!(
            min_distance("intention".to_string(), "execution".to_string()),
            5
        );

        assert_eq!(min_distance("".to_string(), "a".to_string()), 1);
        assert_eq!(min_distance("a".to_string(), "".to_string()), 1);
        assert_eq!(min_distance("a".to_string(), "a".to_string()), 0);
        assert_eq!(min_distance("".to_string(), "".to_string()), 0);
        assert_eq!(min_distance("a".to_string(), "b".to_string()), 1);

        assert_eq!(
            min_distance("distance".to_string(), "springbok".to_string()),
            9
        );
    }
}
