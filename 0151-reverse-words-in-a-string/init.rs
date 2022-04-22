pub fn reverse_words(s: String) -> String {
    let mut v: Vec<&str> = s.split_whitespace().into_iter().collect();
    v.reverse();
    v.join(" ")
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
