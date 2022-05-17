#[allow(clippy::ptr_arg)]
pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    let n = nums.len();
    let k = (k as usize) % n;
    // nums.rotate_right((k as usize) % n);
    if k == 0 {
        return;
    }
    let mut c = n;
    for i in 0..k {
        let mut j = i + k;
        let mut m = nums[i];
        while j != i {
            std::mem::swap(&mut m, &mut nums[j]);
            j = (j + k) % n;
            c -= 1;
        }
        nums[i] = m;
        c -= 1;
        if c == 0 {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(input: &[i32], k: i32, expected: &[i32]) {
        let mut nums = input.to_vec();
        rotate(&mut nums, k);
        assert_eq!(nums, expected, "{:?}", input);
    }
    #[test]
    fn example() {
        tf(&[1, 2, 3, 4, 5, 6, 7], 3, &[5, 6, 7, 1, 2, 3, 4]);

        tf(&[-1], 2, &[-1]);
        tf(&[-1, -100, 3, 99], 2, &[3, 99, -1, -100]);
    }
}
