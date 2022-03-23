use std::collections::HashMap;

pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
    let dict: Vec<_> = word_dict.iter().map(String::as_str).collect();
    let mut memo: HashMap<&str, bool> = HashMap::new();
    recursion(&mut memo, s.as_str(), &dict)
}

fn recursion<'a>(memo: &mut HashMap<&'a str, bool>, s: &'a str, dict: &[&str]) -> bool {
    if s.is_empty() {
        return true;
    }
    if let Some(v) = memo.get(s) {
        return *v;
    }
    let mut v = false;
    for word in dict.iter() {
        if s.starts_with(word) {
            if recursion(memo, &s[word.len()..], dict) {
                v = true;
                break;
            }
        }
    }
    memo.insert(s, v);
    return v;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(s: &str, dict: &[&str], expected: bool) {
        let word_dict: Vec<_> = dict.iter().map(|v| v.to_string()).collect();
        let output = word_break(s.to_string(), word_dict);
        assert_eq!(
            output, expected,
            "{}, output: {}, expected: {}",
            s, output, expected
        );
    }

    #[test]
    fn example() {
        tf("leetcode", &vec!["leet", "code"], true);
        tf("applepenapple", &vec!["apple", "pen"], true);
        tf(
            "catsandog",
            &vec!["cats", "dog", "sand", "and", "cat"],
            false,
        );

        tf(
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab",
            &vec!["a","aa","aaa","aaaa","aaaaa","aaaaaa","aaaaaaa","aaaaaaaa","aaaaaaaaa","aaaaaaaaaa"],
           false
        );
    }
}
