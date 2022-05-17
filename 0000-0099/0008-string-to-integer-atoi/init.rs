pub fn my_atoi(s: String) -> i32 {
    const MAX: i64 = 1 << 31;

    let mut v: i64 = 0;
    let mut is_negative = false;
    let mut iter = s.bytes();

    // while let Some(b) = iter.next() {
    for b in iter.by_ref() {
        match b {
            32 => {
                continue;
            }
            45 => {
                is_negative = true;
                break;
            }
            43 => {
                break;
            }
            48..=57 => {
                v = b as i64 - 48;
                break;
            }
            _ => {
                return 0;
            }
        }
    }

    // while let Some(b) = iter.next() {
    for b in iter {
        match b {
            48..=57 => {
                v = v * 10 + b as i64 - 48;
                if v >= MAX {
                    v = MAX;
                    break;
                }
            }
            _ => {
                break;
            }
        }
    }

    if is_negative {
        (-v) as i32
    } else if v == MAX {
        (v - 1) as i32
    } else {
        v as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(input: &str, expcted: i32) {
        let output = my_atoi(String::from(input));
        assert_eq!(output, expcted);
    }

    #[test]
    fn example() {
        tf("42", 42);
        tf("   -42", -42);
        tf("4193 with words", 4193);
        tf("3333333333", 2147483647);
        tf("-3333333333", -2147483648);

        tf("+-12", 0);

        tf("a123", 0);
        tf("1a123", 1);
    }
}
