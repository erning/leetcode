pub fn is_valid(s: String) -> bool {
    let mut stack: Vec<char> = Vec::new();
    for c in s.chars() {
        match c {
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            ')' | ']' | '}' => {
                if let Some(v) = stack.pop() {
                    if v != c {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            _ => return false,
        }
    }
    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(is_valid("()".to_owned()), true);
        assert_eq!(is_valid("()[]{}".to_owned()), true);
        assert_eq!(is_valid("(]".to_owned()), false);

        assert_eq!(is_valid("(())".to_owned()), true);
        assert_eq!(is_valid("[[][()]]".to_owned()), true);
    }
}
