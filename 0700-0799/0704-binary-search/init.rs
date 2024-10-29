use std::cmp::Ordering;

pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut a = 0;
    let mut b = nums.len() as i32 - 1;
    while a <= b {
        let c = a + (b - a) / 2;
        match nums[c as usize].cmp(&target) {
            Ordering::Less => a = c + 1,
            Ordering::Greater => b = c - 1,
            Ordering::Equal => return c,
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(input: &[i32], target: i32, expected: i32) {
        let nums = input.to_vec();
        let output = search(nums, target);
        assert_eq!(output, expected, "{:?}", (input, target));
    }

    #[test]
    fn example() {
        tf(&[-1, 0, 3, 5, 9, 12], 9, 4);
        tf(&[-1, 0, 3, 5, 9, 12], 2, -1);

        tf(&[1, 2], 1, 0);
        tf(&[1, 2], 2, 1);
        tf(&[1, 2], 3, -1);

        tf(&[5], -5, -1);
        tf(&[2, 5], 0, -1);
    }
}
