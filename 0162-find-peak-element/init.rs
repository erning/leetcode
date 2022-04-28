pub fn find_peak_element(nums: Vec<i32>) -> i32 {
    let mut a = 0;
    let mut b = nums.len() - 1;
    if b == 0 {
        return 0;
    }

    while a + 1 < b {
        let c = a + (b - a + 1) / 2;
        if nums[c] < nums[c - 1] {
            b = c;
        } else {
            a = c;
        }
    }

    return if nums[a] > nums[b] { a } else { b } as i32;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(input: &[i32]) {
        let nums = input.to_vec();
        let output = find_peak_element(nums);
        let i = output as usize;
        if input.len() == 1 {
            assert!(i == 0)
        } else if i == 0 {
            assert!(input[0] > input[1]);
        } else if i == input.len() - 1 {
            assert!(input[i] > input[i - 1]);
        } else {
            assert!(input[i] > input[i - 1]);
            assert!(input[i] > input[i + 1]);
        }
    }

    #[test]
    fn example() {
        tf(&[1, 2, 3, 1]);
        tf(&[1, 2, 1, 3, 5, 6, 4]);

        tf(&[1]);

        tf(&[2, 1]);
        tf(&[1, 2]);
        tf(&[1, 3, 2, 1]);

        tf(&[1, 2, 3, 2, 1]);
        tf(&[1, 2, 1, 2, 1]);
        tf(&[1, 2, 0, 1]);
    }
}
