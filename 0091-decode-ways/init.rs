// use std::collections::HashMap;
use std::collections::HashSet;

pub fn num_decodings(s: String) -> i32 {
    let table: HashSet<&str> = [
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15", "16",
        "17", "18", "19", "20", "21", "22", "23", "24", "25", "26",
    ]
    .iter()
    .copied()
    .collect();

    let len = s.len();
    let mut dp = vec![0; len + 1];
    dp[len] = 1;

    if table.contains(&s[len - 1..len]) {
        dp[len - 1] = dp[len];
    }
    for i in (0..len - 1).rev() {
        if table.contains(&s[i..i + 1]) {
            dp[i] += dp[i + 1];
        }
        if table.contains(&s[i..i + 2]) {
            dp[i] += dp[i + 2];
        }
    }

    dp[0]

    // fn recursion<'a>(memo: &mut HashMap<&'a str, i32>, s: &'a str) -> i32 {
    //     if s.is_empty() {
    //         return 1;
    //     }
    //     if let Some(count) = memo.get(s) {
    //         return *count;
    //     }
    //     let mut count = 0;
    //     for token in TABLE {
    //         if s.starts_with(token) {
    //             count += recursion(memo, &s[token.len()..]);
    //         }
    //     }
    //     memo.insert(s, count);
    //     count
    // }

    // let mut memo: HashMap<&str, i32> = HashMap::new();
    // recursion(&mut memo, &s[..])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(num_decodings("12".to_owned()), 2);
        assert_eq!(num_decodings("226".to_owned()), 3);
        assert_eq!(num_decodings("06".to_owned()), 0);

        assert_eq!(num_decodings("1234".to_owned()), 3);
        assert_eq!(num_decodings("4321".to_owned()), 2);

        assert_eq!(
            num_decodings("111111111111111111111111111111111111111111111".to_owned()),
            1836311903
        );
    }
}
