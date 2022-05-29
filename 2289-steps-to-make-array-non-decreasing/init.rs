pub fn total_steps(nums: Vec<i32>) -> i32 {
    let mut stack: Vec<(i32, i32)> = Vec::new();
    let mut max = 0;

    for &v in nums.iter() {
        let mut step = 0;

        while let Some(&last) = stack.last() {
            if v < last.0 {
                break;
            }
            step = step.max(last.1);
            stack.pop();
        }

        step = if stack.is_empty() { 0 } else { step + 1 };
        if step > max {
            max = step;
        }
        stack.push((v, step));
    }

    max
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(input: &[i32], excepted: i32) {
        let output = total_steps(input.to_vec());
        assert_eq!(output, excepted);
    }

    #[test]
    fn example() {
        tf(&[5, 3, 4, 4, 7, 3, 6, 11, 8, 5, 11], 3);
        tf(&[4, 5, 7, 7, 13], 0);

        tf(&[5, 4, 3, 2, 1], 1);

        tf(&[9, 1, 2, 3, 4, 5, 6, 1, 2, 3], 6);
        tf(&[9, 1, 2, 3, 4, 5, 6, 1, 2, 3, 4, 5, 6, 7, 8], 9);

        tf(&[1, 1, 2, 1, 1], 2);
        tf(&[1, 1, 2, 1, 1, 1], 3);

        tf(&[5, 11, 7, 8, 11], 2);

        tf(&[7, 14, 4, 14, 13, 2, 6, 13], 3);
    }
}
