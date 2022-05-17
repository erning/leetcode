pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.is_empty() {
        return String::new();
    }
    if strs.len() == 1 {
        return strs[0].clone();
    }

    let ss: Vec<&[u8]> = strs.iter().map(|s| s.as_bytes()).collect();
    let mut i = 0;
    'outer: loop {
        let s0 = *ss.first().unwrap();
        if i >= s0.len() {
            break;
        }
        let ch = s0[i];
        for sn in ss.iter().skip(1) {
            if i >= sn.len() || sn[i] != ch {
                break 'outer;
            }
        }
        i += 1;
    }

    strs[0][..i].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            longest_common_prefix(vec![
                "flower".to_owned(),
                "flow".to_owned(),
                "flight".to_owned()
            ]),
            "fl"
        );
        assert_eq!(
            longest_common_prefix(vec![
                "dog".to_owned(),
                "racecar".to_owned(),
                "car".to_owned()
            ]),
            ""
        );

        assert_eq!(longest_common_prefix(vec!["hello".to_owned(),]), "hello");
        assert_eq!(
            longest_common_prefix(vec!["".to_owned(), "hello".to_owned(),]),
            ""
        );
        assert_eq!(
            longest_common_prefix(vec!["hello".to_owned(), "world".to_owned(),]),
            ""
        );
    }
}
