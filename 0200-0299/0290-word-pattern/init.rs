pub fn word_pattern(pattern: String, s: String) -> bool {
    let pv: Vec<usize> = pattern.bytes().map(|v| (v - b'a') as usize).collect();
    let sv: Vec<&str> = s.split_whitespace().collect();
    if pv.len() != sv.len() {
        return false;
    }
    let mut map: Vec<Option<&str>> = vec![None; 26];
    for (i, &p) in pv.iter().enumerate() {
        let s = sv[i];
        if let Some(v) = map[p] {
            if v != s {
                return false;
            }
        } else if map.contains(&Some(s)) {
            return false;
        } else {
            map[p] = Some(s);
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(pattern: &str, s: &str, expected: bool) {
        let output = word_pattern(pattern.to_string(), s.to_string());
        assert_eq!(output, expected, "{:?}", (pattern, s));
    }

    #[test]
    fn example() {
        tf("abba", "dog cat cat dog", true);
        tf("abba", "dog cat cat fish", false);
        tf("aaaa", "dog cat cat dog", false);

        tf("abba", "dog dog dog dog", false);
    }
}
