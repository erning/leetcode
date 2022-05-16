pub fn longest_palindrome(s: String) -> String {
    if s.is_empty() {
        return s;
    }
    let len = s.len();
    let s = s.as_bytes();
    let mut max_len = 0;
    let mut max_a = 0;
    let mut max_b = 0;
    let mut i = 0;

    while i < len - max_len / 2 {
        let mut a = i;
        let mut b = i + 1;
        while b < len && s[i] == s[b] {
            b += 1;
        }
        let c = b;
        b -= 1;
        while a > 0 && b < len - 1 && s[a - 1] == s[b + 1] {
            a -= 1;
            b += 1;
        }

        if max_len < b - a {
            max_len = b - a;
            max_a = a;
            max_b = b;
        }
        i = c
    }

    std::str::from_utf8(&s[max_a..max_b + 1])
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(input: &str, expected: &str) {
        let output = longest_palindrome(String::from(input));
        assert_eq!(output.as_str(), expected)
    }

    #[test]
    fn example() {
        tf("babad", "bab");
        tf("cbbd", "bb");
    }
}
