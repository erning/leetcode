pub fn count_substrings(s: String) -> i32 {
    let s = s.as_bytes();
    let n = s.len();
    let mut dp = vec![vec![false; n]; n];
    for (i, row) in dp.iter_mut().enumerate() {
        row[i] = true;
    }
    let mut count = n;
    for w in 2..=n {
        for i in 0..=(n - w) {
            let j = i + w - 1;
            if s[i] == s[j] && (i + 1 >= j || dp[i + 1][j - 1]) {
                dp[i][j] = true;
                count += 1;
            }
        }
    }
    count as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! tf {
        ($s:expr, $expected:expr) => {
            let output = count_substrings($s.to_string());
            assert_eq!(output, $expected, "{:?}", $s);
        };
    }

    #[test]
    fn example() {
        tf!("abc", 3);
        tf!("aaa", 6);

        tf!("longtimenosee", 14);
    }
}
