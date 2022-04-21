pub fn single_number(nums: Vec<i32>) -> i32 {
    let mut num = 0;
    for i in 0..32 {
        let mut sum = 0;
        for &v in nums.iter() {
            sum += (v >> i) & 1;
        }
        if sum % 3 != 0 {
            num |= 1 << i;
        }
    }
    num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(single_number(vec![2, 2, 3, 2]), 3);
        assert_eq!(single_number(vec![0, 1, 0, 1, 0, 1, 99]), 99);
    }
}
