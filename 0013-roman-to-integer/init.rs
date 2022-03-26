use std::collections::HashMap;

pub fn roman_to_int(s: String) -> i32 {
    let symbols: HashMap<u8, i32> = [
        (b'I', 1),
        (b'V', 5),
        (b'X', 10),
        (b'L', 50),
        (b'C', 100),
        (b'D', 500),
        (b'M', 1000),
    ]
    .iter()
    .map(|v| *v)
    .collect();

    let mut value = 0;
    let mut iter = s.as_bytes().iter();
    let mut next = iter.next();
    let mut pv = 0;
    while let Some(c) = next {
        let v = symbols.get(c).unwrap();
        value += v;
        if *v > pv {
            value -= pv << 1;
        }
        pv = *v;
        next = iter.next();
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
