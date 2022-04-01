pub fn next_permutation(nums: &mut Vec<i32>) {
    let len = nums.len();
    if len <= 1 {
        return;
    }
    let mut p = len - 2;
    loop {
        if p == 0 || nums[p] < nums[p + 1] {
            break;
        }
        p -= 1;
    }
    let m = nums[p];
    let q = nums[p + 1..]
        .iter()
        .enumerate()
        .filter(|&(_, v)| *v > m)
        .reduce(|acc, v| if acc.1 <= v.1 { acc } else { v });
    if let Some((q, _)) = q {
        nums.swap(p + 1 + q, p);
    } else {
        nums.swap(len - 1, p);
    }
    nums[p + 1..].sort();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ft(input: Vec<i32>, expected: Vec<i32>) {
        let mut nums = input;
        next_permutation(&mut nums);
        assert_eq!(nums, expected);
    }

    #[test]
    fn example() {
        ft(vec![1, 2, 3], vec![1, 3, 2]);
        ft(vec![3, 2, 1], vec![1, 2, 3]);
        ft(vec![1, 1, 5], vec![1, 5, 1]);

        ft(vec![1], vec![1]);
        ft(vec![2, 3, 1], vec![3, 1, 2]);

        ft(vec![1, 4, 3, 2], vec![2, 1, 3, 4]);
        ft(vec![5, 4, 7, 5, 3, 2], vec![5, 5, 2, 3, 4, 7]);
    }
}
