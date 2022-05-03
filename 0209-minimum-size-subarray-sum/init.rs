pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut i = 0;
    let mut j = 0;
    let mut min = n + 1;
    let mut sum = 0;

    while i < n {
        while j < n {
            sum += nums[j];
            j += 1;
            if sum >= target {
                if j - i < min {
                    min = j - i;
                }
                break;
            }
        }
        while i < j {
            sum -= nums[i];
            i += 1;
            if sum >= target {
                if j - i < min {
                    min = j - i;
                }
            } else {
                break;
            }
        }
    }

    return if min > n { 0 } else { min as i32 };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(min_sub_array_len(3, vec![1, 1, 1]), 3);
        assert_eq!(min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]), 2);
        assert_eq!(min_sub_array_len(4, vec![1, 4, 4]), 1);
        assert_eq!(min_sub_array_len(11, vec![1, 1, 1, 1, 1, 1, 1, 1]), 0);

        assert_eq!(min_sub_array_len(15, vec![2, 14]), 2);
    }
}
