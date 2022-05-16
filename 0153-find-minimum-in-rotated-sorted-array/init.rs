pub fn find_min(nums: Vec<i32>) -> i32 {
    fn p(nums: &[i32], a: usize, b: usize) -> usize {
        if a + 1 == b {
            return if nums[a] < nums[b] { a } else { b };
        }
        let m = a + (b - a) / 2;
        if nums[a] < nums[m] {
            p(nums, m, b)
        } else {
            p(nums, a, m)
        }
    }
    let p = if nums[0] > nums[nums.len() - 1] {
        p(&nums, 0, nums.len() - 1)
    } else {
        0
    };
    nums[p]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(find_min(vec![3, 4, 5, 1, 2]), 1);
        assert_eq!(find_min(vec![4, 5, 6, 7, 0, 1, 2]), 0);
        assert_eq!(find_min(vec![11, 13, 15, 17]), 11)
    }
}
