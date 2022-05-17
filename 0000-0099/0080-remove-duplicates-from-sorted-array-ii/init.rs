#[allow(clippy::ptr_arg)]
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let len = nums.len();
    if len <= 2 {
        return len as i32;
    }
    let mut k = 2;
    let mut p1 = nums[0];
    let mut p2 = nums[1];
    for n in nums.iter_mut().skip(2) {
        if *n != p1 {
            p1 = p2;
            p2 = *n;
            k += 1
        } else {
            *n = i32::MAX;
        }
    }

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
        let mut nums = input.clone();
        let s = remove_duplicates(&mut nums) as usize;
        assert_eq!(nums[..s], expected, "{:?}", input);
    }

    #[test]
    fn example() {
        tf(vec![1, 1, 1, 2, 2, 3], vec![1, 1, 2, 2, 3]);
        tf(vec![0, 0, 1, 1, 1, 1, 2, 3, 3], vec![0, 0, 1, 1, 2, 3, 3]);
    }
}
