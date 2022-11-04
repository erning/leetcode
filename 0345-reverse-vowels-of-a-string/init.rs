pub fn reverse_vowels(s: String) -> String {
    let s = s.as_bytes();
    let mut out = s.to_owned();
    let mut i = 0;
    let mut j = s.len() - 1;

    fn is_vowel(c: u8) -> bool {
        return c == b'a'
            || c == b'e'
            || c == b'i'
            || c == b'o'
            || c == b'u'
            || c == b'A'
            || c == b'E'
            || c == b'I'
            || c == b'O'
            || c == b'U';
    }

    while i < j {
        if is_vowel(s[i]) {
            while i < j {
                if is_vowel(s[j]) {
                    out[i] = s[j];
                    out[j] = s[i];
                    j -= 1;
                    break;
                }
                j -= 1
            }
        }
        i += 1;
    }

    return std::str::from_utf8(&out).unwrap().to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(reverse_vowels("hello".to_string()), "holle");
        assert_eq!(reverse_vowels("leetcode".to_string()), "leotcede");
    }
}
