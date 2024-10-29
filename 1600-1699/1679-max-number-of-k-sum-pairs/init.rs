use std::cmp::Ordering;
pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
    let mut nums = nums;
    nums.sort_unstable();
    let mut count = 0;
    let mut i = 0;
    let mut j = nums.len() - 1;
    while i < j {
        match (nums[i] + nums[j]).cmp(&k) {
            Ordering::Greater => j -= 1,
            Ordering::Less => i += 1,
            Ordering::Equal => {
                count += 1;
                i += 1;
                j -= 1;
            }
        }
        // let c = nums[i] + nums[j];
        // if c > k {
        //     j -= 1;
        // } else if c < k {
        //     i += 1;
        // } else {
        //     count += 1;
        //     i += 1;
        //     j -= 1;
        // }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(max_operations(vec![1, 2, 3, 4], 5), 2);
        assert_eq!(max_operations(vec![3, 1, 3, 4, 3], 6), 1);
    }
}
