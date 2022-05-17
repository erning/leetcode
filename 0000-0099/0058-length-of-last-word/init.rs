pub fn length_of_last_word(s: String) -> i32 {
    let mut length = 0;
    for &c in s.as_bytes().iter().rev().skip_while(|&v| *v == b' ') {
        if c == b' ' {
            break;
        }
        length += 1;
    }
    length
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(length_of_last_word("Hello World".to_string()), 5);

        assert_eq!(length_of_last_word("World".to_string()), 5);
        assert_eq!(length_of_last_word("Hello World  ".to_string()), 5);
        assert_eq!(length_of_last_word("".to_string()), 0);
        assert_eq!(length_of_last_word(" ".to_string()), 0);
    }
}
