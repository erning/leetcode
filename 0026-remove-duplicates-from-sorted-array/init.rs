#[allow(clippy::ptr_arg)]
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut k = 1;
    let mut prev = nums[0];
    for v in nums.iter_mut().skip(1) {
        if *v != prev {
            prev = *v;
            k += 1;
        } else {
            *v = i32::MAX;
        }
    }

    let len = nums.len();
    let mut i = 1;
    let mut j = 2;
    'outer: loop {
        loop {
            if i >= len - 1 {
                break 'outer;
            }
            if nums[i] == i32::MAX {
                break;
            }
            i += 1;
        }
        if j < i + 1 {
            j = i + 1
        }
        loop {
            if j >= len {
                break 'outer;
            }
            if nums[j] != i32::MAX {
                break;
            }
            j += 1;
        }
        nums.swap(i, j);
        i += 1;
        j += 1;
    }

    k
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(input: Vec<i32>, expected: Vec<i32>) {
        let mut nums = input;
        let k = remove_duplicates(&mut nums) as usize;
        assert_eq!(nums[..k], expected)
    }

    #[test]
    fn example() {
        tf(vec![1, 1, 2], vec![1, 2]);
        tf(vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4], vec![0, 1, 2, 3, 4]);

        tf(vec![1], vec![1]);
        tf(vec![1, 1], vec![1]);

        tf(vec![-3, -1, 0, 0, 0, 3, 3], vec![-3, -1, 0, 3]);
    }
}
