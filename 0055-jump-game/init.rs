pub fn can_jump(nums: Vec<i32>) -> bool {
    let len = nums.len();
    let mut dp: Vec<bool> = vec![false; len];
    dp[0] = true;
    let mut max = 0;
    let mut prev = 0;
    let mut skip = 1;
    for (i, &n) in nums.iter().take(len - 1).enumerate() {
        if !dp[i] {
            if i > max {
                return false;
            }
            prev = 0;
            skip = 1;
            continue;
        }
        let curr = n as usize;
        let k = i + curr;
        if k >= len - 1 {
            return true;
        }
        if k > max {
            max = k;
        }
        if skip < prev {
            skip = prev;
        }
        // for j in i + skip..=k {
        //     dp[j] = true;
        // }
        for v in dp.iter_mut().take(k + 1).skip(i + skip) {
            *v = true;
        }
        prev = curr;
        if skip > 1 {
            skip -= 1;
        }
    }
    dp[len - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(can_jump(vec![2, 3, 1, 1, 4]), true);
        assert_eq!(can_jump(vec![3, 2, 1, 0, 4]), false);
        assert_eq!(can_jump(vec![3, 2, 1, 0, 1, 1, 1]), false);
    }
}
