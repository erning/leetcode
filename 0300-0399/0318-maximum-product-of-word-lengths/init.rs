pub fn max_product(words: Vec<String>) -> i32 {
    let sets: Vec<u32> = words
        .iter()
        .map(|v| v.bytes().fold(0, |acc, it| 1 << (it - b'a') | acc))
        .collect();

    let mut max = 0;
    for (i, a) in words.iter().zip(sets.iter()).enumerate().take(words.len() - 1) {
        for b in words.iter().zip(sets.iter()).skip(i + 1) {
            if a.1 & b.1 != 0 {
                continue;
            }
            let v = a.0.len() * b.0.len();
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
