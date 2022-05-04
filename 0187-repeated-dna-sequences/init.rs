use std::collections::HashSet;

pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
    let mut visited: HashSet<&[u8]> = HashSet::new();
    let mut answer: HashSet<&[u8]> = HashSet::new();
    for sub in s.as_bytes().windows(10).into_iter() {
        if visited.contains(sub) {
            answer.insert(sub);
        } else {
            visited.insert(sub);
        }
    }
    answer
        .into_iter()
        .map(|v| v.to_vec())
        .map(|v| String::from_utf8(v).unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(s: &str, expected: Vec<&str>) {
        let mut output = find_repeated_dna_sequences(s.to_string());
        output.sort_unstable();
        assert_eq!(output, expected);
    }

    #[test]
    fn example() {
        tf(
            "AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT",
            vec!["AAAAACCCCC", "CCCCCAAAAA"],
        );

        tf("AAAAAAAAAAAAA", vec!["AAAAAAAAAA"]);
    }
}
