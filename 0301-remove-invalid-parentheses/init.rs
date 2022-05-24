use std::collections::HashSet;

pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
    let s = s.as_bytes();
    let count = {
        let mut a = 0;
        let mut b = 0;
        for &c in s.iter() {
            if c == b'(' {
                b += 1
            } else if c == b')' {
                if b > 0 {
                    b -= 1;
                } else {
                    a += 1
                }
            }
        }
        a + b
    };

    fn backtrack(
        s: &[u8],
        curr: &[u8],
        answer: &mut HashSet<String>,
        balanced: i32,
        deleted: i32,
        count: i32,
    ) {
        if balanced < 0 || deleted > count {
            return;
        }
        if s.is_empty() {
            if balanced == 0 {
                answer.insert(String::from_utf8(curr.to_vec()).unwrap());
            }
            return;
        }
        let mut next = curr.to_vec();
        next.push(s[0]);
        match s[0] {
            b'(' => {
                backtrack(&s[1..], &next, answer, balanced + 1, deleted, count);
                backtrack(&s[1..], curr, answer, balanced, deleted + 1, count);
            }
            b')' => {
                backtrack(&s[1..], &next, answer, balanced - 1, deleted, count);
                backtrack(&s[1..], curr, answer, balanced, deleted + 1, count);
            }
            _ => {
                backtrack(&s[1..], &next, answer, balanced, deleted, count);
            }
        }
    }

    let mut answer: HashSet<String> = HashSet::new();
    backtrack(s, "".as_bytes(), &mut answer, 0, 0, count);
    answer.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(input: &str, excepted: &[&str]) {
        let mut output = remove_invalid_parentheses(input.to_string());
        output.sort_unstable();
        let mut excepted: Vec<String> = excepted.into_iter().map(|v| v.to_string()).collect();
        excepted.sort_unstable();
        assert_eq!(output, excepted);
    }

    #[test]
    fn example() {
        tf("()())()", &["(())()", "()()()"]);
        tf("(a)())()", &["(a())()", "(a)()()"]);
        tf(")(", &[""])
    }
}
