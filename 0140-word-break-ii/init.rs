use std::collections::HashMap;

pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
    fn recursion<'a>(
        memo: &mut HashMap<&'a str, Vec<Vec<&'a str>>>,
        s: &'a str,
        dict: &[&'a str],
    ) -> Vec<Vec<&'a str>> {
        if s.is_empty() {
            return Vec::new();
        }
        if let Some(rv) = memo.get(s) {
            return rv.clone();
        }
        let mut rv = Vec::new();
        for &word in dict.iter() {
            if s.starts_with(word) {
                if s.len() == word.len() {
                    rv.push(vec![word]);
                    continue;
                }
                for i in recursion(memo, &s[word.len()..], dict) {
                    let mut v = vec![word];
                    v.extend(i.iter());
                    rv.push(v);
                }
            }
        }
        memo.insert(s, rv.clone());
        return rv;
    }

    let dict: Vec<_> = word_dict.iter().map(String::as_str).collect();
    let mut memo: HashMap<&str, Vec<Vec<&str>>> = HashMap::new();
    recursion(&mut memo, s.as_str(), &dict)
        .into_iter()
        .map(|v| v.join(" "))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(s: &str, word_dict: Vec<&str>, expected: Vec<&str>) {
        let word_dict: Vec<String> = word_dict.into_iter().map(|s| s.to_string()).collect();
        let mut output = word_break(s.to_string(), word_dict);
        output.sort();
        assert_eq!(output, expected, "{:?}", s);
    }
    #[test]
    fn example() {
        tf(
            "catsanddog",
            vec!["cat", "cats", "and", "sand", "dog"],
            vec!["cat sand dog", "cats and dog"],
        );
        tf(
            "pineapplepenapple",
            vec!["apple", "pen", "applepen", "pine", "pineapple"],
            vec![
                "pine apple pen apple",
                "pine applepen apple",
                "pineapple pen apple",
            ],
        );
    }
}
