pub fn convert_to_title(column_number: i32) -> String {
    let mut answer: Vec<u8> = Vec::new();
    let mut n = column_number;
    assert!(n >= 1);
    loop {
        n -= 1;
        let m = (n % 26) as u8;
        answer.push(m + b'A');
        n /= 26;
        if n == 0 {
            break;
        }
    }
    answer.reverse();
    String::from_utf8(answer).unwrap_or(String::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(convert_to_title(1), "A");
        assert_eq!(convert_to_title(28), "AB");
        assert_eq!(convert_to_title(701), "ZY");
    }
}
