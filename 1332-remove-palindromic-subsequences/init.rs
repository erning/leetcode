pub fn remove_palindrome_sub(s: String) -> i32 {
    assert!(!s.is_empty());
    let mut iter = s.chars();
    while let (Some(a), Some(b)) = (iter.next(), iter.next_back()) {
        if a != b {
            return 2;
        }
    }
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(remove_palindrome_sub("ababa".to_string()), 1);
        assert_eq!(remove_palindrome_sub("abb".to_string()), 2);
        assert_eq!(remove_palindrome_sub("baabb".to_string()), 2);
    }
}
