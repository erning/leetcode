pub fn find132pattern(nums: Vec<i32>) -> bool {
    let n = nums.len();
    if n < 2 {
        return false;
    }
    let mut i = 0;
    'i: while i < n - 2 {
        let a = nums[i];
        let mut j = i + 1;
        'j: while j < n - 1 {
            let b = nums[j];
            if b <= a {
                i += 1;
                continue 'i;
            }
            let mut k = j + 1;
            'k: while k < n {
                let c = nums[k];
                if c >= b {
                    j += 1;
                    continue 'j;
                }
                if c <= a {
                    k += 1;
                    continue 'k;
                }
                return true;
            }
            break;
        }
        i += 1;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(find132pattern(vec![1, 2, 3, 4]), false);
        assert_eq!(find132pattern(vec![3, 1, 4, 2]), true);
        assert_eq!(find132pattern(vec![-1, 3, 2, 0]), true);

        assert_eq!(find132pattern(vec![1, 4, 0, -1, -2, -3, -1, -2]), true);
        assert_eq!(find132pattern(vec![-2]), false);
    }
}
