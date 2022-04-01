pub fn longest_valid_parentheses(s: String) -> i32 {
    let s = s.as_bytes();
    let mut queue: Vec<(usize, usize)> = Vec::new();
    let mut stack: Vec<usize> = Vec::new();
    for (i, &v) in s.iter().enumerate() {
        if v == b'(' {
            stack.push(i);
            continue;
        }
        if let Some(mut p) = stack.pop() {
            while let Some((a, b)) = queue.last() {
                if p > b + 1 {
                    break;
                }
                if p == b + 1 {
                    p = *a;
                }
                queue.pop();
            }
            queue.push((p, i));
        }
    }
    let max = queue.iter().map(|&(a, b)| b - a + 1).max();
    return if let Some(max) = max { max as i32 } else { 0 };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(longest_valid_parentheses("())()".to_owned()), 2);
        assert_eq!(longest_valid_parentheses("(()".to_owned()), 2);
        assert_eq!(longest_valid_parentheses(")()())".to_owned()), 4);
        assert_eq!(longest_valid_parentheses("".to_owned()), 0);

        assert_eq!(longest_valid_parentheses("()(()".to_owned()), 2);
        assert_eq!(longest_valid_parentheses("()(())".to_owned()), 6);
    }
}
