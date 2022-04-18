use std::collections::HashMap;

pub fn partition(s: String) -> Vec<Vec<String>> {
    fn is_palindrome(s: &str) -> bool {
        if s.len() == 1 {
            return true;
        }
        let mut iter = s.chars();
        while let (Some(a), Some(b)) = (iter.next(), iter.next_back()) {
            if a != b {
                return false;
            }
        }
        true
    }

    fn dfs<'a>(s: &'a str, cache: &mut HashMap<&'a str, Vec<Vec<&'a str>>>) -> Vec<Vec<&'a str>> {
        if s.len() == 1 {
            return vec![vec![s]];
        }
        if let Some(parts) = cache.get(s) {
            return parts.clone();
        }
        let mut parts: Vec<Vec<&str>> = Vec::new();
        if is_palindrome(s) {
            parts.push(vec![s]);
        }
        for i in 1..s.len() {
            let a = &s[..i];
            if !is_palindrome(a) {
                continue;
            }
            let b = &s[i..];
            for mut part in dfs(b, cache) {
                part.push(a);
                parts.push(part);
            }
        }
        cache.insert(s, parts.clone());
        parts
    }

    let mut cache: HashMap<&str, Vec<Vec<&str>>> = HashMap::new();
    let mut parts = dfs(&s, &mut cache);
    parts.iter_mut().for_each(|v| v.reverse());
    parts
        .into_iter()
        .map(|v| v.into_iter().map(|s| s.to_string()).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            partition("aab".to_string()),
            vec![vec!["a", "a", "b"], vec!["aa", "b"]]
        );
        assert_eq!(partition("a".to_string()), vec![vec!["a"]]);
    }
}
