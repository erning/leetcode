pub fn length_of_last_word(s: String) -> i32 {
    const space: u8 = b' ';
    let mut length = 0;
    for &c in s.as_bytes().iter().rev().skip_while(|&v| *v == space) {
        if c == space {
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
