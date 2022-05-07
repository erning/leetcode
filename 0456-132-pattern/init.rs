pub fn find132pattern(nums: Vec<i32>) -> bool {
    let mut c = i32::MIN;
    let mut stack: Vec<i32> = Vec::new();
    for v in nums.into_iter().rev() {
        if v < c {
            return true;
        }
        while let Some(&b) = stack.last() {
            if v <= b {
                break;
            }
            c = stack.pop().unwrap();
        }
        stack.push(v);
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(find132pattern(vec![1, 2, 3, 4]), false);
        assert_eq!(find132pattern(vec![3, 1, 4, 2]), true);
        assert_eq!(find132pattern(vec![-1, 3, 2, 0]), true);

        assert_eq!(find132pattern(vec![1, 4, 0, -1, -2, -3, -1, -2]), true);
        assert_eq!(find132pattern(vec![-2]), false);
    }
}
