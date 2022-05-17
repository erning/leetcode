pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut m = nums[0];
    let mut max = nums[0];
    for n in nums.into_iter().skip(1) {
        m = if m > 0 { m + n } else { n };
        if m > max {
            max = m
        }
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
        assert_eq!(max_sub_array(vec![1]), 1);
        assert_eq!(max_sub_array(vec![5, 4, -1, 7, 8]), 23);
    }
}
