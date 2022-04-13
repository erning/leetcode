pub fn restore_ip_addresses(s: String) -> Vec<String> {
    #[inline]
    fn is_valid_byte(s: &[u8]) -> bool {
        if s.len() > 1 && s[0] == b'0' {
            return false;
        }
        let mut v = 0;
        for &c in s.iter() {
            v = v * 10 + (c - b'0') as u32;
        }
        v <= 255
    }

    let s = s.as_bytes();
    let len = s.len();
    if len < 4 || len > 12 {
        return vec![];
    }
    let mut rv = Vec::new();

    for a in 1..=usize::min(3, len - 3) {
        let s1 = &s[..a];
        if !is_valid_byte(s1) {
            continue;
        }
        for b in a + 1..=usize::min(a + 3, len - 2) {
            let s2 = &s[a..b];
            if !is_valid_byte(s2) {
                continue;
            }
            for c in b + 1..=usize::min(b + 3, len - 1) {
                let s3 = &s[b..c];
                if !is_valid_byte(s3) {
                    continue;
                }
                if len - c > 3 {
                    continue;
                }
                let s4 = &s[c..];
                if !is_valid_byte(s4) {
                    continue;
                }

                let v = format!(
                    "{}.{}.{}.{}",
                    String::from_utf8(s1.to_vec()).unwrap(),
                    String::from_utf8(s2.to_vec()).unwrap(),
                    String::from_utf8(s3.to_vec()).unwrap(),
                    String::from_utf8(s4.to_vec()).unwrap()
                );
                rv.push(v);
            }
        }
    }

    rv
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            restore_ip_addresses("25525511135".to_string()),
            vec!["255.255.11.135", "255.255.111.35"]
        );
        assert_eq!(restore_ip_addresses("0000".to_string()), vec!["0.0.0.0"]);
        assert_eq!(
            restore_ip_addresses("101023".to_string()),
            vec![
                "1.0.10.23",
                "1.0.102.3",
                "10.1.0.23",
                "10.10.2.3",
                "101.0.2.3"
            ]
        );
    }
}
