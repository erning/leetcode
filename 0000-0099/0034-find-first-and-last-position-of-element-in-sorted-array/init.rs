pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // a = index of first target
    let mut c = match nums.binary_search(&target) {
        Err(_) => return vec![-1, -1],
        Ok(i) => i,
    };
    let mut a = 0;
    while a != c {
        match &nums[..c].binary_search(&target) {
            Err(i) => {
                a = *i;
                break;
            }
            Ok(i) => c = *i,
        }
    }
    // b = index of first target+1
    let mut c = match &nums[a + 1..].binary_search(&(target + 1)) {
        Err(i) => return vec![a as i32, (a + i) as i32],
        Ok(i) => a + 1 + i,
    };
    let mut b = a + 1;
    while b != c {
        match &nums[a + 1..c].binary_search(&(target + 1)) {
            Err(i) => {
                b = a + 1 + *i;
                break;
            }
            Ok(i) => c = a + 1 + *i,
        }
    }
    // [a, b-1]
    vec![a as i32, b as i32 - 1]
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
