pub fn max_product(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut max = i32::MIN;
    let mut prev: Vec<i32> = Vec::with_capacity(n);
    for &v in nums.iter() {
        if v > max {
            max = v;
        }
        prev.push(v);
    }
    let mut curr: Vec<i32> = vec![0; n];
    for i in 1..n {
        for (j, v) in curr.iter_mut().enumerate().skip(i) {
            *v = prev[j - 1] * nums[j];
            if *v > max {
                max = *v;
            }
        }
        std::mem::swap(&mut curr, &mut prev);
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
