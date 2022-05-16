use std::cmp::Ordering;

pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    fn f(nums: &[i32], target: i32, a: usize, b: usize) -> Option<usize> {
        let c = a + (b - a) / 2;
        if c == a {
            return if nums[a] == target {
                Some(a)
            } else if nums[b] == target {
                Some(b)
            } else {
                None
            };
        }
        match target.cmp(&nums[c]) {
            Ordering::Less => f(nums, target, a, c - 1),
            Ordering::Greater => f(nums, target, c + 1, b),
            _ => Some(c),
        }
    }

    fn p1(nums: &[i32], target: i32, a: usize, b: usize) -> usize {
        if nums[a] == target {
            return a;
        }
        let c = a + (b - a) / 2;
        if nums[c] == target {
            p1(nums, target, a + 1, c)
        } else {
            p1(nums, target, c + 1, b)
        }
    }

    fn p2(nums: &[i32], target: i32, a: usize, b: usize) -> usize {
        if nums[b] == target {
            return b;
        }
        let c = a + (b - a) / 2;
        if nums[c] == target {
            p2(nums, target, c, b - 1)
        } else {
            p2(nums, target, a, c - 1)
        }
    }

    if nums.is_empty() {
        return vec![-1, -1];
    }
    match f(&nums, target, 0, nums.len() - 1) {
        Some(p) => {
            vec![
                p1(&nums, target, 0, p) as i32,
                p2(&nums, target, p, nums.len() - 1) as i32,
            ]
        }
        None => vec![-1, -1],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(search_range(vec![5, 7, 7, 8, 8, 10], 8), vec![3, 4]);
        assert_eq!(search_range(vec![5, 7, 7, 8, 8, 10], 6), vec![-1, -1]);
        assert_eq!(search_range(vec![], 0), vec![-1, -1]);

        assert_eq!(search_range(vec![1, 2, 3, 3, 3, 3, 4, 5, 9], 3), vec![2, 5]);

        assert_eq!(search_range(vec![5, 7, 7, 8, 8, 10, 10], 11), vec![-1, -1]);
    }
}
