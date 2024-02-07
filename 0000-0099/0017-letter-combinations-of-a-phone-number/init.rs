pub fn letter_combinations(digits: String) -> Vec<String> {
    let map: [Vec<char>; 8] = [
        vec!['a', 'b', 'c'],      // 2
        vec!['d', 'e', 'f'],      // 3
        vec!['g', 'h', 'i'],      // 4
        vec!['j', 'k', 'l'],      // 5
        vec!['m', 'n', 'o'],      // 6
        vec!['p', 'q', 'r', 's'], // 7
        vec!['t', 'u', 'v'],      // 8
        vec!['w', 'x', 'y', 'z'], // 9
    ];
    let mut rv = Vec::<String>::with_capacity(4usize.pow(digits.len() as u32));
    let mut iter = digits.as_bytes().iter();

    if let Some(i) = iter.next() {
        rv.append(
            &mut map[(i - 0x32) as usize]
                .iter()
                .map(|c| c.to_string())
                .collect(),
        );
    } else {
        return rv;
    }

    // while let Some(i) = iter.next() {
    for i in iter {
        let len = rv.len();
        let chars = &map[(i - 0x32) as usize];
        for _ in 1..chars.len() {
            rv.extend_from_within(..len);
        }
        let mut iter = rv.iter_mut();
        for c in chars {
            for _ in 0..len {
                iter.next().unwrap().push(*c);
            }
        }
    }

    rv
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(input: &str, expect: &[&str]) {
        let mut output = letter_combinations(input.to_string());
        output.sort();
        assert_eq!(output, expect, "expect {:?} but {:?}", expect, output);
    }

    #[test]
    fn example() {
        tf(
            "23",
            &["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"],
        );
        tf("", &[]);
        tf("2", &["a", "b", "c"]);
    }
}
