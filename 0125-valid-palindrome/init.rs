pub fn is_palindrome(s: String) -> bool {
    #[inline]
    fn normalize(x: u8) -> u8 {
        if x >= 0x41 && x <= 0x5a {
            return x + 0x20;
        }
        if x >= 0x61 && x <= 0x7a {
            return x;
        }
        if x >= 0x30 && x <= 0x39 {
            return x;
        }
        0
    }

    let mut iter = s.as_bytes().iter();
    let mut opt_a = iter.next();
    let mut opt_b = iter.next_back();
    while let (Some(a), Some(b)) = (opt_a, opt_b) {
        let a = normalize(*a);
        if a == 0 {
            opt_a = iter.next();
            continue;
        }
        let b = normalize(*b);
        if b == 0 {
            opt_b = iter.next_back();
            continue;
        }
        if a != b {
            return false;
        }
        opt_a = iter.next();
        opt_b = iter.next_back();
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            is_palindrome("A man, a plan, a canal: Panama".to_string()),
            true
        );
        assert_eq!(is_palindrome("race a car".to_string()), false);
        assert_eq!(is_palindrome(" ".to_string()), true);

        assert_eq!(is_palindrome("aba".to_string()), true);
    }
}
