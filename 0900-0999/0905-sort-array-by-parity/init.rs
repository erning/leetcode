pub fn sort_array_by_parity(nums: Vec<i32>) -> Vec<i32> {
    let mut nums = nums;
    let mut i = 0;
    let mut j = nums.len() - 1;
    loop {
        while i < j && nums[i] % 2 == 0 {
            i += 1;
        }
        while i < j && nums[j] % 2 == 1 {
            j -= 1;
        }
        if i >= j {
            break;
        }
        nums.swap(i, j);
        i += 1;
        j -= 1;
    }
    nums
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(nums: &[i32]) {
        let input = nums.to_vec();
        let output = sort_array_by_parity(input);
        let mut i = 0;
        while i < output.len() && output[i] % 2 == 0 {
            i += 1;
        }
        while i < output.len() && output[i] % 2 == 1 {
            i += 1;
        }
        assert_eq!(i, output.len());
    }

    #[test]
    fn example() {
        tf(&[3, 1, 2, 4]);
        tf(&[0]);
    }
}
