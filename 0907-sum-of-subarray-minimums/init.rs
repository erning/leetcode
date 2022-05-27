pub fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
    let mut answer = 0u64;
    let n = arr.len();
    let mut dp = vec![i32::MAX; n + 1];
    for i in 0..n {
        for j in 1..=n - i {
            let v = i32::min(dp[j - 1], arr[i + j - 1]);
            dp[j] = v;
            answer += v as u64;
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
        assert_eq!(sum_subarray_mins(vec![3, 1, 2, 4]), 17);
        assert_eq!(sum_subarray_mins(vec![11, 81, 94, 43, 3]), 444);
    }
}
