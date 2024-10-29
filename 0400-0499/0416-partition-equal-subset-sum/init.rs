use std::collections::HashMap;

pub fn can_partition(nums: Vec<i32>) -> bool {
    fn find(
        target: i32,
        nums: &[i32],
        used: &mut Vec<bool>,
        memo: &mut HashMap<i32, bool>,
    ) -> bool {
        if target == 0 {
            return true;
        }
        if target < 0 {
            return false;
        }
        if let Some(&v) = memo.get(&target) {
            return v;
        }
        let mut v = false;
        for (i, n) in nums.iter().enumerate() {
            if used[i] {
                continue;
            }
            used[i] = true;
            if find(target - n, nums, used, memo) {
                v = true;
                break;
            }
            used[i] = false;
        }
        memo.insert(target, v);
        v
    }

    let sum: i32 = nums.iter().sum();
    if sum % 2 == 1 {
        return false;
    }
    let target = sum / 2;
    let mut used = vec![false; nums.len()];
    let mut memo: HashMap<i32, bool> = HashMap::new();

    find(target, &nums, &mut used, &mut memo)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(can_partition(vec![1, 5, 11, 5]), true);
        assert_eq!(can_partition(vec![1, 2, 3, 5]), false);
    }
}
