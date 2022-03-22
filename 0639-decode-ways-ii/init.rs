use std::collections::HashSet;

pub fn num_decodings(s: String) -> i32 {
    const MOD: i32 = 10i32.pow(9) + 7;
    const TABLE: [&str; 26] = [
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15", "16",
        "17", "18", "19", "20", "21", "22", "23", "24", "25", "26",
    ];
    let table: HashSet<&str> = TABLE.iter().map(|v| *v).collect();

    let len = s.len();
    let mut dp = vec![0; len + 1];
    dp[len] = 1;

    fn expend1(s: &str) -> Vec<&str> {
        if s != "*" {
            return vec![s];
        }
        return vec!["1", "2", "3", "4", "5", "6", "7", "8", "9"];
    }

    fn expend2(s: &str) -> Vec<&str> {
        match (&s[0..1], &s[1..2]) {
            ("*", "*") => TABLE
                .iter()
                .map(|v| *v)
                .filter(|v| v.len() == 2)
                .filter(|v| &v[1..2] != "0")
                .collect(),
            (x, "*") => TABLE
                .iter()
                .map(|v| *v)
                .filter(|v| v.len() == 2)
                .filter(|v| &v[1..2] != "0")
                .filter(|v| &v[0..1] == x)
                .collect(),
            ("*", x) => TABLE
                .iter()
                .map(|v| *v)
                .filter(|v| v.len() == 2)
                .filter(|v| &v[0..1] != "0")
                .filter(|v| &v[1..2] == x)
                .collect(),
            _ => vec![s],
        }
    }

    {
        let i = len - 1;
        for ss in expend1(&s[i..i + 1]) {
            if table.contains(ss) {
                dp[i] += dp[i + 1];
            }
        }
    }
    for i in (0..len - 1).rev() {
        for ss in expend1(&s[i..i + 1]) {
            if table.contains(ss) {
                dp[i] += dp[i + 1];
                dp[i] %= MOD;
            }
        }
        for ss in expend2(&s[i..i + 2]) {
            if table.contains(&ss) {
                dp[i] += dp[i + 2];
                dp[i] %= MOD;
            }
        }
    }

    dp[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(num_decodings("*".to_owned()), 9);
        assert_eq!(num_decodings("1*".to_owned()), 18);
        assert_eq!(num_decodings("2*".to_owned()), 15);

        assert_eq!(num_decodings("*1*1*0".to_owned()), 404);
        assert_eq!(num_decodings("*********".to_owned()), 291868912);
    }
}
