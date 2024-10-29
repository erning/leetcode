pub fn has_all_codes(s: String, k: i32) -> bool {
    let mask = (1 << k) - 1;
    let mut set = vec![false; (1 << k) as usize];
    let mut value = 0;
    s.bytes()
        .take(k as usize)
        .map(|v| (v - b'0') as usize)
        .for_each(|v| value = (value << 1) | v);
    set[value] = true;
    for v in s.bytes().skip(k as usize).map(|v| (v - b'0') as usize) {
        value = (value << 1) & mask | v;
        set[value] = true;
    }
    !set.iter().any(|&v| !v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(has_all_codes("00110110".to_string(), 2), true);
        assert_eq!(has_all_codes("0110".to_string(), 1), true);
        assert_eq!(has_all_codes("0110".to_string(), 2), false);
    }
}
