use std::collections::HashMap;

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map: HashMap<Vec<u8>, Vec<usize>> = HashMap::new();

    for (i, s) in strs.iter().enumerate() {
        let mut k = s.as_bytes().to_vec();
        k.sort();
        if let Some(v) = map.get_mut(&k) {
            v.push(i);
        } else {
            map.insert(k, vec![i]);
        }
    }

    map.values()
        .map(|v| v.into_iter().map(|i| strs[*i].clone()).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(strs: Vec<&str>, expected: Vec<Vec<&str>>) {
        let input: Vec<String> = strs.iter().map(|s| s.to_string()).collect();
        let mut output = group_anagrams(input);
        output.iter_mut().for_each(|v| v.sort());
        output.sort();
        let mut expected = expected;
        expected.iter_mut().for_each(|v| v.sort());
        expected.sort();
        assert_eq!(output, expected);
    }

    #[test]
    fn example() {
        tf(
            vec!["eat", "tea", "tan", "ate", "nat", "bat"],
            vec![vec!["bat"], vec!["nat", "tan"], vec!["ate", "eat", "tea"]],
        );
    }
}
