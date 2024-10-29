pub fn rearrange_characters(s: String, target: String) -> i32 {
    let mut s_map: [i32; 26] = [0; 26];
    s.bytes().for_each(|v| s_map[(v - b'a') as usize] += 1);
    let mut t_map: [i32; 26] = [0; 26];
    target.bytes().for_each(|v| t_map[(v - b'a') as usize] += 1);

    s_map
        .iter()
        .zip(t_map.iter())
        .filter(|(_, &t)| t != 0)
        .map(|(&s, &t)| s / t)
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(input: &str, target: &str, expected: i32) {
        let output = rearrange_characters(input.to_string(), target.to_string());
        assert_eq!(output, expected, "{:?}", (input, target));
    }

    #[test]
    fn example() {
        tf("ilovecodingonleetcode", "code", 2);
        tf("abcba", "abc", 1);
        tf("abbaccaddaeea", "aaaaa", 1);

        tf("abc", "abcd", 0);
    }
}
