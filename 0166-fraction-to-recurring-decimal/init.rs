use std::collections::HashMap;

pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
    let mut s = String::new();
    if numerator.signum() * denominator.signum() < 0 {
        s.push('-')
    }
    let n: i64 = (numerator as i64).abs();
    let d: i64 = (denominator as i64).abs();
    s.push_str((n / d).to_string().as_str());
    let mut m = n % d;
    if m == 0 {
        return s;
    }

    let mut digits: Vec<u8> = Vec::new();
    let mut map: HashMap<i64, usize> = HashMap::new();
    let mut infinite = None;
    while m != 0 && infinite.is_none() {
        map.insert(m, digits.len());
        m *= 10;
        let digit = (m / d) as u8;
        digits.push(digit);
        m %= d;
        infinite = map.get(&m);
    }

    s.push('.');
    if let Some(&i) = infinite {
        for &v in &digits[..i] {
            s.push(char::from_digit(v as u32, 10).unwrap());
        }
        s.push('(');
        for &v in &digits[i..] {
            s.push(char::from_digit(v as u32, 10).unwrap());
        }
        s.push(')');
    } else {
        for &v in digits.iter() {
            s.push(char::from_digit(v as u32, 10).unwrap());
        }
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(fraction_to_decimal(1, 2), "0.5");
        assert_eq!(fraction_to_decimal(2, 1), "2");
        assert_eq!(fraction_to_decimal(4, 333), "0.(012)");

        assert_eq!(
            fraction_to_decimal(-1, -2147483648),
            "0.0000000004656612873077392578125"
        );
    }
}
