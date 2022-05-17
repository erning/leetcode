use std::collections::HashSet;

pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    let mut set: HashSet<i32> = HashSet::new();
    let k = k as usize;
    for n in nums.iter().take(k + 1) {
        if set.contains(n) {
            return true;
        }
        set.insert(*n);
    }
    for (i, n) in nums.iter().skip(k + 1).enumerate() {
        set.remove(&nums[i]);
        if set.contains(n) {
            return true;
        }
        set.insert(*n);
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(contains_nearby_duplicate(vec![1, 2, 3, 1], 3), true);
        assert_eq!(contains_nearby_duplicate(vec![1, 0, 1, 1], 1), true);
        assert_eq!(contains_nearby_duplicate(vec![1, 2, 3, 1, 2, 3], 2), false);

        assert_eq!(contains_nearby_duplicate(vec![1, 2, 3, 4], 0), false);
        assert_eq!(contains_nearby_duplicate(vec![1, 1, 3, 4], 0), false);
        assert_eq!(contains_nearby_duplicate(vec![1, 2, 3, 4], 1), false);
        assert_eq!(contains_nearby_duplicate(vec![1, 1, 3, 4], 1), true);
    }
}
