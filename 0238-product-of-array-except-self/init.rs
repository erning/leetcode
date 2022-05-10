pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    assert!(nums.len() >= 2);
    let mut zeros = 0;
    let mut p = 1;
    for v in nums.iter() {
        if *v == 0 {
            zeros += 1;
            if zeros > 1 {
                return vec![0; nums.len()];
            }
            continue;
        }
        p *= v;
    }
    if zeros > 0 {
        nums.into_iter()
            .map(|v| if v == 0 { p } else { 0 })
            .collect()
    } else {
        nums.into_iter().map(|v| p / v).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(input: &[i32], expected: &[i32]) {
        let output = product_except_self(input.to_vec());
        assert_eq!(output, expected, "{:?}", input);
    }

    #[test]
    fn example() {
        tf(&[1, 2, 3, 4], &[24, 12, 8, 6]);
        tf(&[-1, 1, 0, -3, 3], &[0, 0, 9, 0, 0]);
        tf(&[-1, 0, 0, -3, 3], &[0, 0, 0, 0, 0]);
    }
}
