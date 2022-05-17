pub fn find_duplicate(nums: Vec<i32>) -> i32 {
    let mut dup: Vec<bool> = vec![false; nums.len()];
    for v in nums {
        let i = v as usize - 1;
        if dup[i] {
            return v;
        }
        dup[i] = true;
    }
    0
}

fn _find_duplicate(nums: Vec<i32>) -> i32 {
    let mut a = 1;
    let mut b = nums.len() - 1;
    while a <= b {
        let m = a + (b - a) / 2;
        let c = nums.iter().filter(|&&v| v as usize <= m).count();
        if c <= m {
            a = m + 1;
        } else {
            b = m - 1;
        }
    }
    a as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(find_duplicate(vec![1, 3, 4, 2, 2]), 2);
        assert_eq!(find_duplicate(vec![3, 1, 3, 4, 2]), 3);

        assert_eq!(find_duplicate(vec![2, 2, 2, 2, 2]), 2);
        assert_eq!(find_duplicate(vec![1, 2, 2]), 2);
        assert_eq!(find_duplicate(vec![1, 4, 4, 2, 4]), 4);
    }
}
