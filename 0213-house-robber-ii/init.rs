pub fn rob(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    if n == 1 {
        return nums[0];
    }
    let f = |(a, b), num| (b, i32::max(num + a, b));
    i32::max(
        nums[0..n - 1].iter().fold((0, 0), f).1,
        nums[1..].iter().fold((0, 0), f).1,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(rob(vec![2, 3, 2]), 3);
        assert_eq!(rob(vec![1, 2, 3, 1]), 4);

        assert_eq!(rob(vec![1]), 1);
    }
}
