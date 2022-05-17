pub fn is_anagram(s: String, t: String) -> bool {
    let mut map = [0; 26];
    for &c in s.as_bytes() {
        map[(c - b'a') as usize] += 1;
    }
    for &c in t.as_bytes() {
        map[(c - b'a') as usize] -= 1;
    }
    // map.iter().find(|&&c| c != 0).is_none()
    !map.iter().any(|&c| c != 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            is_anagram("anagram".to_string(), "nagaram".to_string()),
            true
        );
        assert_eq!(is_anagram("rat".to_string(), "car".to_string()), false);
    }
}
