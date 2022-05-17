pub fn majority_element(nums: Vec<i32>) -> i32 {
    // The majority element is the element that appears more than ⌊n / 2⌋ times.
    assert_ne!(nums.len(), 0);
    let mut m = nums[0];
    let mut c = 1;
    for &n in nums.iter().skip(1) {
        if c == 0 {
            m = n;
        }
        if m == n {
            c += 1
        } else {
            c -= 1
        }
    }
    m
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(majority_element(vec![3, 2, 3]), 3);
        assert_eq!(majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
    }
}
