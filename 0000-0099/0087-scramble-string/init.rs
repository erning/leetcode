use std::collections::HashMap;

pub fn is_scramble(s1: String, s2: String) -> bool {
    fn is_scramble<'a>(
        s1: &'a [u8],
        s2: &'a [u8],
        memo: &mut HashMap<(&'a [u8], &'a [u8]), bool>,
    ) -> bool {
        let len = s1.len();
        if len == 1 {
            return s1 == s2;
        }
        if s1 == s2 {
            return true;
        }
        let k = (s1, s2);
        if let Some(&v) = memo.get(&k) {
            return v;
        }
        let mut f1 = [0; 26];
        let mut f2 = [0; 26];
        for &c in s1 {
            f1[(c - b'a') as usize] += 1;
        }
        for &c in s2 {
            f2[(c - b'a') as usize] += 1;
        }
        if f1 != f2 {
            memo.insert(k, false);
            return false;
        }
        let mut v = false;
        for p in 1..len {
            if is_scramble(&s1[..p], &s2[..p], memo) && is_scramble(&s1[p..], &s2[p..], memo) {
                v = true;
                break;
            }
            if is_scramble(&s1[..p], &s2[len - p..], memo)
                && is_scramble(&s1[p..], &s2[..len - p], memo)
            {
                v = true;
                break;
            }
        }
        memo.insert(k, v);
        v
    }

    let mut memo: HashMap<(&[u8], &[u8]), bool> = HashMap::new();
    is_scramble(s1.as_bytes(), s2.as_bytes(), &mut memo)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(is_scramble("great".to_string(), "rgeat".to_string()), true);
        assert_eq!(is_scramble("abcde".to_string(), "caebd".to_string()), false);
        assert_eq!(is_scramble("a".to_string(), "a".to_string()), true);
    }
}
