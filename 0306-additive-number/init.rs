pub fn is_additive_number(num: String) -> bool {
    fn backtrack(nums: &[i32], fibs: &[i32], answer: &mut bool) {
        if *answer {
            return;
        }
        let n = fibs.len();
        if n >= 3 {
            if fibs[n - 3] + fibs[n - 2] != fibs[n - 1] {
                return;
            }
            if nums.is_empty() {
                *answer = true;
                return;
            }
        }
        let mut a = 0;
        for (i, &b) in nums.iter().enumerate() {
            if i > 0 && a == 0 {
                return;
            }
            a = a * 10 + b;
            let mut fibs = fibs.to_vec();
            fibs.push(a);
            backtrack(&nums[(i + 1)..], &fibs, answer);
        }
    }

    let nums: Vec<i32> = num.as_bytes().iter().map(|&c| (c - b'0') as i32).collect();
    let mut answer: bool = false;
    backtrack(&nums, &[], &mut answer);
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
    }
}
