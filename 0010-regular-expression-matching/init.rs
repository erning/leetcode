pub fn is_match(s: String, p: String) -> bool {
    let len = s.len();

    let mut prev: Vec<bool> = vec![false; len + 1];
    let mut curr: Vec<bool> = vec![false; len + 1];
    let mut next: Vec<bool> = vec![false; len + 1];
    curr[0] = true;
    let mut pp = 0;

    for cp in p.as_bytes().iter() {
        if *cp == b'*' {
            next[0] = prev[0];
        }
        for (j, cs) in s.as_bytes().iter().enumerate() {
            next[j + 1] = match cp {
                b'*' => prev[j + 1] || (curr[j] || next[j]) && (pp == b'.' || pp == *cs),
                b'.' => curr[j],
                _ => curr[j] && cp == cs,
            };
        }
        pp = *cp;
        std::mem::swap(&mut prev, &mut curr);
        std::mem::swap(&mut curr, &mut next);
        next.iter_mut().for_each(|v| *v = false);
    }

    curr[len]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(is_match("aa".to_owned(), "a".to_owned()), false);
        assert_eq!(is_match("aa".to_owned(), "a*".to_owned()), true);
        assert_eq!(is_match("ab".to_owned(), ".*".to_owned()), true);

        assert_eq!(is_match("aab".to_owned(), "c*a*b".to_owned()), true);
        assert_eq!(is_match("aaa".to_owned(), "ab*a".to_owned()), false);
        assert_eq!(
            is_match("mississippi".to_owned(), "mis*is*p*.".to_owned()),
            false
        );
        assert_eq!(is_match("aaa".to_owned(), "ab*ac*a".to_owned()), true);
        assert_eq!(is_match("a".to_owned(), "ab*a".to_owned()), false);

        assert_eq!(is_match("aaab".to_owned(), "a*b".to_owned()), true);
        assert_eq!(is_match("aabb".to_owned(), "a*b".to_owned()), false);
        assert_eq!(is_match("babb".to_owned(), "a*b".to_owned()), false);
    }
}
