pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let len = nums.len();
    if len == 0 {
        return 0;
    }

    let mut i = 0;
    let mut j = len - 1;
    'outer: loop {
        loop {
            if i >= j {
                break 'outer;
            }
            if nums[i] == val {
                break;
            }
            i += 1;
        }
        loop {
            if j <= i {
                break 'outer;
            }
            if nums[j] != val {
                break;
            }
            j -= 1;
        }
        nums.swap(i, j);
        i += 1;
        j -= 1;
    }

    if nums[i] == val {
        i as i32
    } else {
        (i + 1) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(input: Vec<i32>, val: i32, expected: Vec<i32>) {
        let mut nums = input;
        let k = remove_element(&mut nums, val) as usize;
        let mut output = nums[..k].to_vec();
        output.sort_unstable();
        let mut expected = expected;
        expected.sort_unstable();
        assert_eq!(output, expected)
    }

    #[test]
    fn example() {
        tf(vec![3, 2, 2, 3], 3, vec![2, 2]);
        tf(vec![0, 1, 2, 2, 3, 0, 4, 2], 2, vec![0, 1, 4, 0, 3]);

        tf(vec![4, 5], 4, vec![5]);

        tf(vec![], 2, vec![]);
        tf(vec![2], 2, vec![]);
        tf(vec![2, 2], 2, vec![]);
    }
}
