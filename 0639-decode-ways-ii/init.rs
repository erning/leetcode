pub fn num_decodings(s: String) -> i32 {
    const MOD: u64 = 10u64.pow(9) + 7;

    macro_rules! times_1 {
        ($x:expr) => {
            match $x {
                "*" => 9,
                "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => 1,
                _ => 0,
            }
        };
    }
    macro_rules! times_2 {
        ($x:expr) => {
            match (&$x[0..1], &$x[1..2]) {
                ("*", "*") => 15,
                ("*", "0" | "1" | "2" | "3" | "4" | "5" | "6") => 2,
                ("*", "7" | "8" | "9") => 1,
                ("1", "*") => 9,
                ("2", "*") => 6,
                ("1", "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9") => 1,
                ("2", "0" | "1" | "2" | "3" | "4" | "5" | "6") => 1,
                _ => 0,
            }
        };
    }

    let len = s.len();
    let mut dp = vec![0; len + 1];

    dp[len] = 1;
    dp[len - 1] = times_1!(&s[len - 1..len]);

    for i in (0..len - 1).rev() {
        dp[i] += dp[i + 1] * times_1!(&s[i..i + 1]);
        dp[i] += dp[i + 2] * times_2!(&s[i..i + 2]);
        dp[i] %= MOD;
    }

    dp[0] as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(num_decodings("**".to_owned()), 96);
        assert_eq!(num_decodings("*".to_owned()), 9);
        assert_eq!(num_decodings("1*".to_owned()), 18);
        assert_eq!(num_decodings("2*".to_owned()), 15);
        assert_eq!(num_decodings("0*".to_owned()), 0);
        assert_eq!(num_decodings("3*".to_owned()), 9);

        assert_eq!(num_decodings("*10*1".to_owned()), 99);
        assert_eq!(
            num_decodings("7*9*3*6*3*0*5*4*9*7*3*7*1*8*3*2*0*0*6*".to_owned()),
            196465252
        );

        assert_eq!(num_decodings("*1*1*0".to_owned()), 404);
        assert_eq!(num_decodings("*********".to_owned()), 291868912);
    }
}
