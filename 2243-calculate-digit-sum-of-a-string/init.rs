pub fn digit_sum(s: String, k: i32) -> String {
    // assert!(k >= 2 && s.len() > 1);
    let mut s: Vec<u8> = s.as_bytes().into_iter().map(|v| v - 48).collect();
    let k = k as usize;

    fn sum(s: &[u8]) -> Vec<u8> {
        let mut sum: i32 = s.iter().map(|&v| v as i32).sum();
        if sum == 0 {
            return vec![0];
        }
        let mut v: Vec<u8> = Vec::new();
        while sum > 0 {
            v.push((sum % 10) as u8);
            sum /= 10;
        }
        v.reverse();
        v
    }

    while s.len() > k {
        let mut ns: Vec<u8> = Vec::new();
        let mut i = 0;
        for _ in 0..s.len() / k {
            let group = &s[i..i + k];
            ns.append(&mut sum(group));
            i += k;
        }
        if i < s.len() {
            let group = &s[i..];
            ns.append(&mut sum(group));
        }
        s = ns;
    }

    s = s.into_iter().map(|v| v + 48).collect();
    String::from_utf8(s).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(digit_sum("11111222223".to_string(), 3), "135");
        assert_eq!(digit_sum("00000000".to_string(), 3), "000");
    }
}
