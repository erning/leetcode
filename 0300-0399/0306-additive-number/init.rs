pub fn is_additive_number(num: String) -> bool {
    fn backtrack(nums: &[i32], fibs: [i32; 3], answer: &mut bool) {
        if *answer {
            return;
        }
        if fibs[0] >= 0 {
            if fibs[0] + fibs[1] != fibs[2] {
                return;
            }
            if nums.is_empty() {
                *answer = true;
                return;
            }
        }
        let mut fibs = [fibs[1], fibs[2], 0];
        for (i, &b) in nums.iter().enumerate() {
            if i > 0 && fibs[2] == 0 {
                return;
            }
            fibs[2] = fibs[2] * 10 + b;
            backtrack(&nums[(i + 1)..], fibs, answer);
        }
    }

    let nums: Vec<i32> = num.as_bytes().iter().map(|&c| (c - b'0') as i32).collect();
    let mut answer = false;
    backtrack(&nums, [-1, -1, -1], &mut answer);
    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(is_additive_number("112358".to_string()), true);
        assert_eq!(is_additive_number("199100199".to_string()), true);

        assert_eq!(is_additive_number("1023".to_string()), false);
        assert_eq!(is_additive_number("101".to_string()), true);
        assert_eq!(is_additive_number("000".to_string()), true);
    }
}
