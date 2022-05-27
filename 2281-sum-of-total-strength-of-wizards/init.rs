pub fn total_strength(strength: Vec<i32>) -> i32 {
    let mut answer = 0u64;
    let n = strength.len();
    let mut dp = vec![(i32::MAX, 0); n + 1];
    for i in 0..n {
        for j in 1..=n - i {
            let x = strength[i + j - 1];
            let (min, sum) = dp[j - 1];
            let y = (i32::min(min, x), sum + x);
            dp[j] = y;
            answer += y.0 as u64 * y.1 as u64;
            answer %= 1000000007;
        }
    }

    answer as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(total_strength(vec![1, 3, 1, 2]), 44);
        assert_eq!(total_strength(vec![5, 4, 6]), 213);

        assert_eq!(total_strength(vec![1, 2, 3, 100000, 100000]), 1799803);
        assert_eq!(total_strength(vec![99999, 99999]), 999199731);
    }
}
