pub fn shortest_palindrome(s: String) -> String {
    let t = s.as_bytes();
    let n = s.len();
    let mut max = 1;
    let mut i = n;
    while i > 0 {
        i -= 1;
        let c = i / 2;
        let mut is_palindrome = true;
        for j in 0..=c {
            if t[j] != t[i - j] {
                is_palindrome = false;
                break;
            }
        }
        if is_palindrome {
            max = i + 1;
            break;
        }
    }
    let mut answer = String::new();
    for j in (max..n).rev() {
        answer.push(t[j] as char);
    }
    answer.push_str(s.as_str());
    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(input: &str, expected: &str) {
        let s = input.to_string();
        let output = shortest_palindrome(s);
        assert_eq!(output, expected, "input={:?}", input);
    }

    #[test]
    fn example() {
        tf("aacecaaa", "aaacecaaa");
        tf("abcd", "dcbabcd");
    }
}
