pub fn length_of_longest_substring(s: String) -> i32 {
    let mut max = 0;
    let mut m: [i32; 255] = [0; 255];
    let mut p = 0;
    for (i, b) in s.bytes().enumerate() {
        let b = b as usize;
        if m[b] > p {
            p = m[b]
        }
        let i = i as i32;
        m[b] = i + 1;
        let length = i - p + 1;
        if length > max {
            max = length;
        }
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(s: String, expected: i32) {
        let output = length_of_longest_substring(s);
        assert_eq!(output, expected);
    }

    #[test]
    fn example() {
        tf(String::from("abcabcbb"), 3);
        tf(String::from("bbbbb"), 1);
        tf(String::from("pwwkew"), 3);
    }
}
