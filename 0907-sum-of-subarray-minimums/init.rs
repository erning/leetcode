pub fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
    const M: u64 = 1e9 as u64 + 7;
    let n = arr.len();
    let mut answer = 0u64;

    let mut prev = vec![0; n];
    let mut next = vec![n - 1; n];
    let mut stack = Vec::new();
    for (i, &v) in arr.iter().enumerate() {
        while let Some(&j) = stack.last() {
            if v > arr[j] {
                prev[i] = j + 1;
                break;
            }
            next[j] = i - 1;
            stack.pop();
        }
        stack.push(i);
    }

    for (i, &v) in arr.iter().enumerate() {
        let a = (i - prev[i] + 1) as u64;
        let b = (next[i] - i + 1) as u64;
        let c = v as u64;
        answer += a * b * c;
        answer %= M;
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
