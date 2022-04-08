pub fn add_binary(a: String, b: String) -> String {
    let mut rv: Vec<u8> = Vec::with_capacity(usize::max(a.len(), b.len()) + 1);
    let mut iter_a = a.as_bytes().into_iter().rev();
    let mut iter_b = b.as_bytes().into_iter().rev();
    let mut opt_a = iter_a.next();
    let mut opt_b = iter_b.next();
    let mut c = 0;
    while opt_a.is_some() || opt_b.is_some() {
        let a = *opt_a.unwrap_or(&b'0') - b'0';
        let b = *opt_b.unwrap_or(&b'0') - b'0';
        let v = a + b + c;
        c = v /  2;
        rv.push((v % 2) + b'0');
        opt_a = iter_a.next();
        opt_b = iter_b.next();
    }
    if c > 0 {
        rv.push(c + b'0');
    }
    rv.reverse();
    String::from_utf8(rv).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(add_binary("11".to_string(), "1".to_string()), "100");
        assert_eq!(add_binary("1010".to_string(), "1011".to_string()), "10101");
    }
}
