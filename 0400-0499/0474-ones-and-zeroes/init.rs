pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
    let oz: Vec<(_, _)> = strs
        .iter()
        .map(|s| {
            let o: usize = s.chars().filter(|&c| c == '1').count();
            let z = s.len() - o;
            (o, z)
        })
        .collect();
    let n = n as usize;
    let m = m as usize;
    let mut dp = vec![vec![0; m + 1]; n + 1];
    for (o, z) in oz {
        for i in (o..=n).rev() {
            for j in (z..=m).rev() {
                dp[i][j] = usize::max(dp[i][j], dp[i - o][j - z] + 1);
            }
        }
    }
    dp[n][m] as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(input: &[&str], m: i32, n: i32, expected: i32) {
        let strs: Vec<_> = input.iter().map(|v| v.to_string()).collect();
        let output = find_max_form(strs, m, n);
        assert_eq!(output, expected, "{:?}", input);
    }

    #[test]
    fn example() {
        tf(&["10", "0001", "111001", "1", "0"], 5, 3, 4);
        tf(&["10", "0", "1"], 1, 1, 2);

        tf(
            &[
                "11", "11", "0", "0", "10", "1", "1", "0", "11", "1", "0", "111", "11111000", "0",
                "11", "000", "1", "1", "0", "00", "1", "101", "001", "000", "0", "00", "0011", "0",
                "10000",
            ],
            90,
            66,
            29,
        );

        tf(
            &[
                "0", "11", "1000", "01", "0", "101", "1", "1", "1", "0", "0", "0", "0", "1", "0",
                "0110101", "0", "11", "01", "00", "01111", "0011", "1", "1000", "0", "11101", "1",
                "0", "10", "0111",
            ],
            9,
            80,
            17,
        );
    }
}
