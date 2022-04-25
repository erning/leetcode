pub fn find_closest_number(nums: Vec<i32>) -> i32 {
    let mut m = (i32::MAX, i32::MIN);
    for v in nums.into_iter() {
        let d = v.abs();
        if d < m.0 || (d == m.0 && v > m.1) {
            m = (d, v);
        }
    }
    m.1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(find_closest_number(vec![-4, -2, 1, 4, 8]), 1);
        assert_eq!(find_closest_number(vec![2, -1, 1]), 1);

        assert_eq!(find_closest_number(vec![-10, 10]), 10);
        assert_eq!(find_closest_number(vec![-10, -10]), -10);
    }
}
