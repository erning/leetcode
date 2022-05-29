pub fn max_product(words: Vec<String>) -> i32 {
    fn has_common(a: &[bool; 26], b: &[bool; 26]) -> bool {
        for i in 0..26 {
            if a[i] && b[i] {
                return true;
            }
        }
        return false;
    }

    let words: Vec<(usize, [bool; 26])> = words
        .into_iter()
        .map(|v| {
            let mut set = [false; 26];
            v.bytes().for_each(|v| set[(v - b'a') as usize] = true);
            (v.len(), set)
        })
        .collect();

    let mut max = 0;
    for (i, a) in words.iter().enumerate().take(words.len() - 1) {
        for b in words.iter().skip(i + 1) {
            if has_common(&a.1, &b.1) {
                continue;
            }
            let v = a.0 * b.0;
            if v > max {
                max = v;
            }
        }
    }
    max as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(input: &[&str], expected: i32) {
        let words: Vec<String> = input.iter().map(|v| v.to_string()).collect();
        let output = max_product(words);
        assert_eq!(output, expected);
    }

    #[test]
    fn example() {
        tf(&["abcw", "baz", "foo", "bar", "xtfn", "abcdef"], 16);
        tf(&["a", "ab", "abc", "d", "cd", "bcd", "abcd"], 4);
        tf(&["a", "aa", "aaa", "aaaa"], 0);
    }
}
