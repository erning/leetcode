pub fn str_str(haystack: String, needle: String) -> i32 {
    if needle.is_empty() {
        return 0;
    }

    fn compute_lps(needle: &[u8]) -> Vec<usize> {
        let mut lps = vec![0; needle.len()];
        let mut len = 0;
        let mut i = 1;
        while i < needle.len() {
            if needle[i] == needle[len] {
                len += 1;
                lps[i] = len;
                i += 1;
            } else {
                if len != 0 {
                    len = lps[len - 1];
                } else {
                    lps[i] = 0;
                    i += 1;
                }
            }
        }
        lps
    }

    let haystack = haystack.as_bytes();
    let needle = needle.as_bytes();
    let m = needle.len();
    let n = haystack.len();
    let lps = compute_lps(needle);

    let mut j = 0;
    let mut i = 0;
    while i < n {
        if needle[j] == haystack[i] {
            i += 1;
            j += 1;
        }
        if j == m {
            return i as i32 - j as i32;
            // j = lsp[j-1];
        } else if i < n && needle[j] != haystack[i] {
            if j != 0 {
                j = lps[j - 1];
            } else {
                i += 1
            }
        }
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(str_str("hello".to_owned(), "ll".to_owned()), 2);
        assert_eq!(str_str("aaaaa".to_owned(), "bba".to_owned()), -1);
        assert_eq!(str_str("".to_owned(), "".to_owned()), 0);

        assert_eq!(str_str("".to_owned(), "a".to_owned()), -1);

        assert_eq!(str_str("a".to_owned(), "a".to_owned()), 0);
        assert_eq!(str_str("b".to_owned(), "a".to_owned()), -1);

        assert_eq!(
            str_str("ABABDABACDABABCABAB".to_owned(), "ABABCABAB".to_owned()),
            10
        );
    }
}
