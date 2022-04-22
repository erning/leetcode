pub fn find_min(nums: Vec<i32>) -> i32 {
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
            return p(a, c, nums);
        } else {
            let v = p(a, c, nums);
            return if v.is_some() { v } else { p(c, b, nums) };
        }
    }

    if nums.len() < 7 {
        return nums.into_iter().min().unwrap()
    }

    let pivot = p(0, nums.len() - 1, &nums).unwrap_or(0);
    nums[pivot]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(find_min(vec![1, 3, 5]), 1);
        assert_eq!(find_min(vec![2, 2, 2, 0, 1]), 0);
    }
}
