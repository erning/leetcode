pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
    let mut dp = vec![0; triangle.len() + 1];
    dp[0] = i32::MAX;
    dp[1] = triangle[0][0];
    for (i, row) in triangle.iter().enumerate().skip(1) {
        dp[i + 1] = dp[i] + row.last().unwrap();
        for (j, &v) in row.iter().enumerate().take(i).rev() {
            dp[j + 1] = i32::min(dp[j], dp[j + 1]) + v;
        }
    }
    dp.into_iter().skip(1).min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            minimum_total(vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]]),
            11
        );
        assert_eq!(minimum_total(vec![vec![-10]]), -10);
    }
}
