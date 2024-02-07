pub fn reverse_words(s: String) -> String {
    let mut v: Vec<&str> = s.split_whitespace().collect();
    let mut rs = v.pop().unwrap().to_string();
    while let Some(s) = v.pop() {
        rs.push(' ');
        rs.push_str(s);
    }
    rs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            reverse_words("the sky is blue".to_string()),
            "blue is sky the"
        );
        assert_eq!(reverse_words("  hello world  ".to_string()), "world hello");
        assert_eq!(
            reverse_words("a good   example".to_string()),
            "example good a"
        );
    }
}
