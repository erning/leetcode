pub fn is_isomorphic(s: String, t: String) -> bool {
    const NONE: usize = 256;
    let mut sm: Vec<usize> = vec![NONE; 256];
    let mut tm: Vec<usize> = vec![NONE; 256];
    let s = s.as_bytes();
    let t = t.as_bytes();

    for i in 0..s.len() {
        let si = s[i] as usize;
        let ti = t[i] as usize;

        let sv = &mut sm[si];
        if *sv == NONE {
            *sv = ti;
        } else if *sv != ti {
            return false;
        }

        let tv = &mut tm[ti];
        if *tv == NONE {
            *tv = si;
        } else if *tv != si {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(is_isomorphic("egg".to_string(), "odd".to_string()), true);
        assert_eq!(is_isomorphic("foo".to_string(), "bar".to_string()), false);
        assert_eq!(
            is_isomorphic("paper".to_string(), "title".to_string()),
            true
        );

        assert_eq!(is_isomorphic("badc".to_string(), "baba".to_string()), false);
    }
}
