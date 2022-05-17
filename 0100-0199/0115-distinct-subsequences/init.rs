pub fn num_distinct(s: String, t: String) -> i32 {
    let s = s.as_bytes();
    let t = t.as_bytes();
    let mut prev = vec![1; s.len() + 1];
    for (j, &b) in t.iter().enumerate() {
        let mut curr = vec![0; s.len() + 1];
        for (i, &a) in s.iter().enumerate().skip(j) {
            curr[i + 1] = if a == b { curr[i] + prev[i] } else { curr[i] }
        }
        prev = curr;
    }
    prev[s.len()]
}

// fn dfs(s: &[u8], t: &[u8]) -> i32 {
//     if t.is_empty() {
//         1
//     } else if s.is_empty() {
//         0
//     } else if s[0] == t[0] {
//         dfs(&s[1..], &t[1..]) + dfs(&s[1..], t)
//     } else {
//         dfs(&s[1..], t)
//     }
// }
// return dfs(s.as_bytes(), t.as_bytes());

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(num_distinct("rabbbit".to_string(), "rabbit".to_string()), 3);
        assert_eq!(num_distinct("babgbag".to_string(), "bag".to_string()), 5);

        assert_eq!(num_distinct("".to_string(), "".to_string()), 1);
        assert_eq!(num_distinct("a".to_string(), "".to_string()), 1);
        assert_eq!(num_distinct("".to_string(), "a".to_string()), 0);
    }
}
