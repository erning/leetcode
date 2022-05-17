pub fn roman_to_int(s: String) -> i32 {
    let mut value = 0;
    let mut pv = i32::MAX;
    for c in s.as_bytes().iter() {
        let v = match c {
            b'I' => 1,
            b'V' => 5,
            b'X' => 10,
            b'L' => 50,
            b'C' => 100,
            b'D' => 500,
            b'M' => 1000,
            _ => {
                unimplemented!()
            }
        };
        value += v;
        if v > pv {
            value -= pv << 1;
        }
        pv = v;
    }
    value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(roman_to_int("III".to_owned()), 3);
        assert_eq!(roman_to_int("LVIII".to_owned()), 58);
        assert_eq!(roman_to_int("MCMXCIV".to_owned()), 1994);
    }
}
