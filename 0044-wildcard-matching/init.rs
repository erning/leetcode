pub fn is_match(s: String, p: String) -> bool {
    let len = s.len();

    let mut prev: Vec<bool> = vec![false; len + 1];
    let mut curr: Vec<bool> = vec![false; len + 1];
    prev[0] = true;

    for cp in p.as_bytes().iter() {
        if *cp == b'*' {
            curr[0] = prev[0];
        }
        for (j, cs) in s.as_bytes().iter().enumerate() {
            curr[j + 1] = match cp {
                b'*' => prev[j] || prev[j + 1] || curr[j],
                b'?' => prev[j],
                _ => prev[j] && cp == cs,
            };
        }
        std::mem::swap(&mut prev, &mut curr);
        curr.iter_mut().for_each(|v| *v = false);
    }

    prev[len]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(is_match("aa".to_owned(), "a".to_owned()), false);
        assert_eq!(is_match("aa".to_owned(), "*".to_owned()), true);
        assert_eq!(is_match("cb".to_owned(), "?a".to_owned()), false);

        assert_eq!(is_match("aab".to_owned(), "c*a*b".to_owned()), false);
        assert_eq!(is_match("aaa".to_owned(), "ab*a".to_owned()), false);
        assert_eq!(is_match("adceb".to_owned(), "*a*b".to_owned()), true);

        assert_eq!(is_match("aa".to_owned(), "aa*".to_owned()), true);
        assert_eq!(is_match("aa".to_owned(), "*a".to_owned()), true);
        assert_eq!(is_match("aa".to_owned(), "*b".to_owned()), false);
    }
}
