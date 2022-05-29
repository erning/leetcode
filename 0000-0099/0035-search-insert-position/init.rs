use std::cmp::Ordering;

pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let mut a: i32 = 0;
    let mut b: i32 = nums.len() as i32 - 1;
    while a <= b {
        let c = a + (b - a) / 2;
        match nums[c as usize].cmp(&target) {
            Ordering::Less => a = c + 1,
            Ordering::Greater => b = c - 1,
            Ordering::Equal => return c,
        }
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(search_insert(vec![1, 3, 5, 6], 5), 2);
        assert_eq!(search_insert(vec![1, 3, 5, 6], 2), 1);
        assert_eq!(search_insert(vec![1, 3, 5, 6], 7), 4);

        assert_eq!(search_insert(vec![1, 3], 0), 0);
        assert_eq!(search_insert(vec![1, 3], 2), 1);
        assert_eq!(search_insert(vec![3, 5, 7, 9, 10], 8), 3);

        assert_eq!(search_insert(vec![1, 3, 5, 6], 0), 0);
    }
}
