use std::collections::HashSet;

pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let nums: HashSet<i32> = nums.into_iter().collect();
    let mut visited: HashSet<i32> = HashSet::new();

    let mut longest = 0;
    for &n in nums.iter() {
        if visited.contains(&n) {
            continue;
        }
        let mut m = n + 1;
        while nums.contains(&m) {
            visited.insert(m);
            m += 1;
        }
        let count = m - n;
        if count > longest {
            longest = count;
        }
    }

    longest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(longest_consecutive(vec![1, 2, 3, -1, 0, 4]), 4);

        assert_eq!(longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
        assert_eq!(longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]), 9);
    }
}
