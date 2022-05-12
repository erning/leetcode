pub fn nth_ugly_number(n: i32) -> i32 {
    let n = n as usize;
    let mut dp: Vec<i32> =Vec::new();
    dp.push(1);
    let mut c2 = 0;
    let mut c3 = 0;
    let mut c5 = 0;
    for _ in 1..n {
        let v2 = 2 * dp[c2];
        let v3 = 3 * dp[c3];
        let v5 = 5 * dp[c5];
        let v = v2.min(v3).min(v5);
        if v == v2 {
            c2 += 1;
        }
        if v == v3 {
            c3 += 1;
        }
        if v == v5 {
            c5 += 1;
        }
        dp.push(v);
    }
    *dp.last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(nth_ugly_number(10), 12);
        assert_eq!(nth_ugly_number(1), 1);

        assert_eq!(nth_ugly_number(20), 36);
        assert_eq!(nth_ugly_number(1690), 2123366400);
    }
}
