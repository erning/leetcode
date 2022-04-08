pub fn is_number(s: String) -> bool {
    let s = s.as_bytes();

    let mut accept_sign = true;
    let mut accept_dot = true;
    let mut accept_e = false;
    let mut accepted_e = false;
    let mut require_more = true;

    for &c in s.iter() {
        if c == b'+' || c == b'-' {
            if !accept_sign {
                return false;
            }
            accept_sign = false;
            require_more = true;
            continue;
        }
        if c == b'.' {
            if !accept_dot {
                return false;
            }
            accept_dot = false;
            accept_sign = false;
            continue;
        }
        if c == b'e' || c == b'E' {
            if !accept_e {
                return false;
            }
            accepted_e = true;
            accept_e = false;
            accept_dot = false;
            accept_sign = true;
            require_more = true;
            continue;
        }
        if c >= b'0' && c <= b'9' {
            if accept_sign {
                accept_sign = false;
            }
            if !accepted_e {
                accept_e = true;
            }
            if require_more {
                require_more = false;
            }
            continue;
        }
        return false;
    }

    !require_more
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let strs = vec![
            "2",
            "0089",
            "-0.1",
            "+3.14",
            "4.",
            "-.9",
            "2e10",
            "-90E3",
            "3e+7",
            "+6e-1",
            "53.5e93",
            "-123.456e789",
        ];
        for s in strs.into_iter() {
            assert_eq!(is_number(s.to_string()), true, "{}", &s);
        }
        let strs = vec!["abc", "1a", "1e", "e3", "99e2.5", "--6", "-+3", "95a54e53"];
        for s in strs.into_iter() {
            assert_eq!(is_number(s.to_string()), false, "{}", &s);
        }

        assert_eq!(is_number(".-4".to_string()), false);
    }
}
