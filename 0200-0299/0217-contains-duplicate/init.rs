pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let n = nums.len();
    let mut set: Vec<i32> = vec![i32::MAX; n];
    for v in nums.into_iter() {
        let mut m = (v + 1000000000) as usize % n;
        loop {
            let x = &mut set[m];
            if *x == v {
                return true;
            }
            if *x == i32::MAX {
                *x = v;
                break;
            }
            m = (m + 1) % n;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(contains_duplicate(vec![1, 2, 3, 1]), true);
        assert_eq!(contains_duplicate(vec![1, 2, 3, 4]), false);
        assert_eq!(contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]), true);
    }
}
