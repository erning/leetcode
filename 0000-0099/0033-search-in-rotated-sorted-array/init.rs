use std::cmp::Ordering;

pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    if nums.len() < 7 {
        for (i, v) in nums.iter().enumerate() {
            if *v == target {
                return i as i32;
            }
        }
        return -1;
    }

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

    fn f(nums: &[i32], target: i32, p: usize, a: usize, b: usize) -> Option<usize> {
        if a == b {
            let m = (a + p) % nums.len();
            return if target == nums[m] { Some(m) } else { None };
        }
        let c = a + (b - a) / 2;
        let m = (c + p) % nums.len();
        match target.cmp(&nums[m]) {
            Ordering::Less => f(nums, target, p, a, c - 1),
            Ordering::Greater => f(nums, target, p, c + 1, b),
            _ => Some(m),
        }
    }

    let p = if nums[0] > nums[nums.len() - 1] {
        p(&nums, 0, nums.len() - 1)
    } else {
        0
    };
    if let Some(x) = f(&nums, target, p, 0, nums.len() - 1) {
        x as i32
    } else {
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
        assert_eq!(search(vec![1], 0), -1);

        assert_eq!(search(vec![1, 3], 4), -1);
        assert_eq!(search(vec![1, 3, 5], 2), -1);
        assert_eq!(search(vec![4, 5, 6, 7, 8, 1, 2, 3], 8), 4);
        assert_eq!(search(vec![1, 2, 3, 4, 5, 6, 7], 1), 0);
        assert_eq!(search(vec![2, 3, 4, 5, 6, 7, 1], 1), 6);

        assert_eq!(search(vec![1], 1), 0);
        assert_eq!(search(vec![1, 2], 1), 0);
        assert_eq!(search(vec![1, 2], 2), 1);
    }
}
