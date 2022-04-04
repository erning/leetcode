pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
    let nums: Vec<i32> = nums.into_iter().filter(|&v| v > 0).collect();
    let len = nums.len();

    let mut s: Vec<bool> = vec![false; len];
    for n in nums.iter() {
        let n = *n as usize;
        if n > len {
            continue;
        }
        s[n - 1] = true;
    }
    for (i, &v) in s.iter().enumerate() {
        if v == false {
            return i as i32 + 1;
        }
    }
    len as i32 + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(first_missing_positive(vec![1, 2, 0]), 3);
        assert_eq!(first_missing_positive(vec![3, 4, -1, 1]), 2);
        assert_eq!(first_missing_positive(vec![7, 8, 9, 11, 12]), 1);

        assert_eq!(first_missing_positive(vec![1]), 2);
    }
}
