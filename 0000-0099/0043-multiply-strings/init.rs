pub fn multiply(num1: String, num2: String) -> String {
    fn add(n1: &[u8], n2: &[u8]) -> Vec<u8> {
        let mut rv = vec![0u8; usize::max(n1.len(), n2.len()) + 1];
        let mut iter1 = n1.iter();
        let mut iter2 = n2.iter();
        let mut c = 0;
        for v in rv.iter_mut().rev() {
            let a = *iter1.next_back().unwrap_or(&0);
            let b = *iter2.next_back().unwrap_or(&0);
            let m = a + b + c;
            *v = m % 10;
            c = m / 10;
        }
        rv
    }

    fn mul(num: &[u8], t: u8) -> Vec<u8> {
        let mut rv = vec![0u8; num.len() + 1];
        let mut iter = num.iter();
        let mut c = 0;
        for v in rv.iter_mut().rev() {
            let n = *iter.next_back().unwrap_or(&0);
            let m = n * t + c;
            *v = m % 10;
            c = m / 10;
        }
        rv
    }

    let num1: Vec<u8> = num1.as_bytes().iter().map(|v| v - b'0').collect();
    let num2: Vec<u8> = num2.as_bytes().iter().map(|v| v - b'0').collect();

    let mut rv = vec![];
    for (i, &t) in num2.iter().rev().enumerate() {
        let mut v = mul(&num1, t);
        v.append(&mut vec![0; i]);
        rv = add(&rv, &v);
    }
    rv = rv
        .into_iter()
        .skip_while(|v| *v == 0)
        .map(|v| v + b'0')
        .collect();

    if rv.is_empty() {
        rv = vec![b'0'];
    }
    String::from_utf8(rv).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(multiply("2".to_string(), "3".to_string()), "6");
        assert_eq!(multiply("123".to_string(), "456".to_string()), "56088");

        assert_eq!(multiply("140".to_string(), "721".to_string()), "100940");

        assert_eq!(multiply("0".to_string(), "123456".to_string()), "0");
        assert_eq!(multiply("123456".to_string(), "0".to_string()), "0");

        assert_eq!(multiply("1001".to_string(), "101".to_string()), "101101");
        assert_eq!(multiply("0".to_string(), "0".to_string()), "0");
    }
}
