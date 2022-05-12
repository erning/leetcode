pub fn add_digits(num: i32) -> i32 {
    let mut n = num;
    while n >= 10 {
        let mut m = 0;
        while n > 0 {
            m += n % 10;
            n /= 10;
        }
        n = m;
    }
    n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(add_digits(38), 2);
        assert_eq!(add_digits(0), 0);
        assert_eq!(add_digits(i32::MAX), 1);
    }
}
