pub fn single_number(nums: Vec<i32>) -> i32 {
    nums.into_iter().fold(0, |a, i| a ^ i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(single_number(vec![2, 2, 1]), 1);
        assert_eq!(single_number(vec![4, 1, 2, 1, 2]), 4);
    }
}
