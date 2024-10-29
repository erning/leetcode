pub fn number_of_steps(num: i32) -> i32 {
    let mut step = num & 0x01;
    let mut n = num >> 1;
    while n > 0 {
        step += (n & 0x01) + 1;
        n >>= 1;
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

        assert_eq!(number_of_steps(0), 0);
    }
}
