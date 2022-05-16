pub fn remove_duplicates(s: String, k: i32) -> String {
    let mut stack: Vec<(u8, i32)> = Vec::with_capacity(s.len());
    for &c in s.as_bytes().iter() {
        let n = if let Some(&(a, b)) = stack.last() {
            if a == c {
                b
            } else {
                0
            }
        } else {
            0
        };
        if n + 1 >= k {
            for _ in 0..n {
                stack.pop();
            }
        } else {
            stack.push((c, n + 1));
        }
    }
    let mut answer = String::with_capacity(stack.len());
    for (v, _) in stack.into_iter() {
        answer.push(v as char);
    }
    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(input: &str, k: i32, expected: &str) {
        let output = remove_duplicates(input.to_string(), k);
        assert_eq!(output, expected);
    }

    #[test]
    fn example() {
        tf("abcd", 2, "abcd");
        tf("deeedbbcccbdaa", 3, "aa");
        tf("pbbcggttciiippooaais", 2, "ps");
    }
}
