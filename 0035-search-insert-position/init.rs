pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let nums = nums.as_slice();
    let mut a = 0;
    let mut b = nums.len() - 1;

    loop {
        let c = a + (b - a) / 2;
        if c == a {
            return if target <= nums[c] {
                c as i32
            } else if target <= nums[b] {
                b as i32
            } else {
                b as i32 + 1
            };
        }
        if target < nums[c] {
            b = c - 1;
        } else if target > nums[c] {
            a = c + 1;
        } else {
            break c as i32;
        }
    }
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
