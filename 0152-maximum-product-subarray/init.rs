pub fn max_product(nums: Vec<i32>) -> i32 {
    let mut max = nums[0];
    let mut curr_min = max;
    let mut curr_max = max;
    for v in nums.into_iter().skip(1) {
        let a = curr_min * v;
        let b = curr_max * v;
        curr_min = v.min(a).min(b);
        curr_max = v.max(a).max(b);
        if curr_max > max {
            max = curr_max;
        }
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(max_product(vec![2, 3, -2, 4]), 6);
        assert_eq!(max_product(vec![-2, 0, -1]), 0);

        assert_eq!(max_product(vec![-2]), -2);
    }
}
