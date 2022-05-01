pub fn backspace_compare(s: String, t: String) -> bool {
    let s = s.as_bytes();
    let t = t.as_bytes();

    let mut si = s.len();
    let mut ti = t.len();
    let mut ss = 0;
    let mut ts = 0;

    while si > 0 && ti > 0 {
        let a = s[si - 1];
        let b = t[ti - 1];
        if a == b'#' || b == b'#' {
            if a == b'#' {
                ss += 1;
                si -= 1;
            }
            if b == b'#' {
                ts += 1;
                ti -= 1;
            }
            continue;
        }
        if ss > 0 || ts > 0 {
            if ss > 0 {
                ss -= 1;
                si -= 1;
            }
            if ts > 0 {
                ts -= 1;
                ti -= 1;
            }
            continue;
        }
        if a != b {
            return false;
        }
        ti -= 1;
        si -= 1;
    }

    while si > 0 {
        let a = s[si - 1];
        if a == b'#' {
            ss += 1;
            si -= 1;
            continue;
        }
        if ss > 0 {
            ss -= 1;
            si -= 1;
            continue;
        }
        break;
    }
    while ti > 0 {
        let b = t[ti - 1];
        if b == b'#' {
            ts += 1;
            ti -= 1;
            continue;
        }
        if ts > 0 {
            ts -= 1;
            ti -= 1;
            continue;
        }
        break;
    }

    si == ti
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            backspace_compare("ab#c".to_string(), "ac".to_string()),
            true
        );
        assert_eq!(
            backspace_compare("ab##".to_string(), "c#d#".to_string()),
            true
        );
        assert_eq!(backspace_compare("a#c".to_string(), "b".to_string()), false);
    }
}
