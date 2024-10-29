pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
    let mut m1 = nums[0];
    let mut max = nums[0];
    let mut m2 = nums[0];
    let mut min = nums[0];
    let mut sum = nums[0];
    for n in nums.into_iter().skip(1) {
        m1 = if m1 > 0 { m1 + n } else { n };
        m2 = if m2 < 0 { m2 + n } else { n };
        if m1 > max {
            max = m1;
        }
        if m2 < min {
            min = m2;
        }
        sum += n;
    }
    if max < 0 {
        max
    } else {
        i32::max(max, sum - min)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(max_subarray_sum_circular(vec![1, -2, 3, -2]), 3);
        assert_eq!(max_subarray_sum_circular(vec![5, -3, 5]), 10);
        assert_eq!(max_subarray_sum_circular(vec![-3, -2, -3]), -2);

        assert_eq!(max_subarray_sum_circular(vec![6, -4, -6, 9]), 15);
        assert_eq!(max_subarray_sum_circular(vec![9, 8, 4, -7]), 21);
    }
}
