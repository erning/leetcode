pub fn number_of_steps(num: i32) -> i32 {
    let mut step = 0;
    let mut n = num;
    while n > 0 {
        step += 1;
        if n & 0x01 == 0 {
            n >>= 1;
        } else {
            n -= 1;
        }
    }
    step
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(number_of_steps(14), 6);
        assert_eq!(number_of_steps(8), 4);
        assert_eq!(number_of_steps(123), 12);
    }
}
