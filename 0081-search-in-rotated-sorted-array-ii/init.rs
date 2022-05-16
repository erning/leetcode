use std::cmp::Ordering;

pub fn search(nums: Vec<i32>, target: i32) -> bool {
    let len = nums.len();
    if len < 2 {
        for (_, v) in nums.iter().enumerate() {
            if *v == target {
                return true;
            }
        }
        return false;
    }

    fn p(a: usize, b: usize, nums: &[i32]) -> Option<usize> {
        let va = nums[a];
        let vb = nums[b];
        if a + 1 == b {
            return if va > vb { Some(b) } else { None };
        }
        let c = a + (b - a) / 2;
        let vc = nums[c];
        if va < vc {
            return p(c, b, nums);
        }
        if vb > vc {
            p(a, c, nums)
        } else {
            let v = p(a, c, nums);
            if v.is_some() {
                v
            } else {
                p(c, b, nums)
            }
        }
    }

    let pivot = p(0, len - 1, &nums).unwrap_or(0);
    let mut a = 0;
    let mut b = len - 1;
    while a + 1 < b {
        let c = a + (b - a) / 2;
        let vc = nums[(c + pivot) % len];
        match target.cmp(&vc) {
            Ordering::Less => b = c - 1,
            Ordering::Greater => a = c + 1,
            Ordering::Equal => return true,
        }
    }

    nums[(a + pivot) % len] == target || nums[(b + pivot) % len] == target
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(search(vec![2, 5, 6, 0, 0, 1, 2], 0), true);
        assert_eq!(search(vec![2, 5, 6, 0, 0, 1, 2], 3), false);
        assert_eq!(
            search(
                vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 13, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                13
            ),
            true
        );
        assert_eq!(search(vec![1, 0, 1, 1, 1], 0), true);
        assert_eq!(search(vec![2, 2, 2, 3, 2, 2, 2], 3), true);
        assert_eq!(search(vec![1, 3, 5], 0), false);
        assert_eq!(
            search(
                vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1],
                2
            ),
            true
        );

        assert_eq!(search(vec![1], 1), true);
        assert_eq!(search(vec![1], 2), false);

        assert_eq!(search(vec![1, 2], 1), true);
        assert_eq!(search(vec![1, 2], 2), true);
        assert_eq!(search(vec![2, 1], 1), true);
        assert_eq!(search(vec![2, 1], 2), true);
        assert_eq!(search(vec![2, 1], 3), false);

        assert_eq!(search(vec![1, 1, 1, 1], 1), true);
        assert_eq!(search(vec![1, 1, 1, 2], 1), true);
    }
}
