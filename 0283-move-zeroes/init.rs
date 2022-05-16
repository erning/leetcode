#[allow(clippy::ptr_arg)]
pub fn move_zeroes(nums: &mut Vec<i32>) {
    let mut z = nums.len();
    for (i, &v) in nums.iter().enumerate() {
        if v == 0 {
            z = i;
            break;
        }
    }
    for i in z + 1..nums.len() {
        if nums[i] != 0 {
            nums[z] = nums[i];
            z += 1;
        }
    }
    for v in nums.iter_mut().skip(z) {
        *v = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(input: &[i32], expected: &[i32]) {
        let mut nums = input.to_vec();
        move_zeroes(&mut nums);
        assert_eq!(nums, expected, "{:?}", input);
    }

    #[test]
    fn example() {
        tf(&[0, 1, 0, 3, 12], &[1, 3, 12, 0, 0]);
        tf(&[0], &[0]);
        tf(&[1], &[1]);
    }
}
