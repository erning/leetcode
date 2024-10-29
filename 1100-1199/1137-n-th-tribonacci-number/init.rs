pub fn tribonacci(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    let mut dp: [i32; 3] = [0, 1, 1];
    let mut m = n - 2;
    while m > 0 {
        let sum: i32 = dp.iter().sum();
        dp.rotate_left(1);
        dp[2] = sum;
        m -= 1;
    }
    dp[2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(tribonacci(4), 4);
        assert_eq!(tribonacci(25), 1389537);

        assert_eq!(tribonacci(37), 2082876103);
    }
}
