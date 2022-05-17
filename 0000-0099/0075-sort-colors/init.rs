#[allow(clippy::ptr_arg)]
pub fn sort_colors(nums: &mut Vec<i32>) {
    let mut a = 0;
    let mut b = nums.len() - 1;
    if a == b {
        return;
    }
    while a < b && nums[a] == 0 {
        a += 1;
    }
    while a < b && nums[b] == 2 {
        b -= 1;
    }
    let mut c = a;
    while c <= b {
        if nums[c] == 0 {
            nums[c] = nums[a];
            nums[a] = 0;
            while a < b && nums[a] == 0 {
                a += 1;
            }
            if a >= b {
                break;
            }
            if a > c {
                c = a;
            }
        } else if nums[c] == 2 {
            nums[c] = nums[b];
            nums[b] = 2;
            while a < b && nums[b] == 2 {
                b -= 1;
            }
            if a >= b {
                break;
            }
        } else {
            c += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(input: Vec<i32>) {
        let mut output = input.clone();
        sort_colors(&mut output);
        let mut expected = input.clone();
        expected.sort_unstable();
        assert_eq!(output, expected, "{:?}", input);
    }
    #[test]
    fn example() {
        tf(vec![1, 2, 0, 0, 2, 1]);
        tf(vec![2, 0, 2, 1, 1, 0]);
        tf(vec![2, 0, 1]);

        tf(vec![0, 0]);
    }
}
