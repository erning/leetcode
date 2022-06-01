pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut rs = 0;
    nums.into_iter()
        .map(|v| {
            rs += v;
            rs
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(input: &[i32], expected: &[i32]) {
        let output = running_sum(input.to_vec());
        assert_eq!(output, expected, "{:?}", input);
    }
    #[test]
    fn example() {
        tf(&[1, 2, 3, 4], &[1, 3, 6, 10]);
        tf(&[1, 1, 1, 1, 1], &[1, 2, 3, 4, 5]);
        tf(&[3, 1, 2, 10, 1], &[3, 4, 6, 16, 17]);
    }
}
