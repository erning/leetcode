pub fn rob(nums: Vec<i32>) -> i32 {
    // let mut a = 0;
    // let mut b = 0;
    // for num in nums.into_iter() {
    //     (a, b) = (b, i32::max(num + a, b));
    // }
    // b
    nums.into_iter()
        .fold((0, 0), |(a, b), num| (b, i32::max(num + a, b)))
        .1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(rob(vec![1, 2, 3, 1]), 4);
        assert_eq!(rob(vec![2, 7, 9, 3, 1]), 12);

        assert_eq!(rob(vec![2, 1]), 2);
        assert_eq!(rob(vec![1]), 1);
        assert_eq!(rob(vec![]), 0);
    }
}
