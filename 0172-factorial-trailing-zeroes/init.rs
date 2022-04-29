pub fn trailing_zeroes(n: i32) -> i32 {
    let mut f2 = 0;
    let mut f5 = 0;
    for v in 1..=(n as usize) {
        let mut v = v as i32;
        while v % 2 == 0 {
            v /= 2;
            f2 += 1;
        }
        while v % 5 == 0 {
            v /= 5;
            f5 += 1;
        }
    }
    i32::min(f2, f5)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(trailing_zeroes(3), 0);
        assert_eq!(trailing_zeroes(5), 1);
        assert_eq!(trailing_zeroes(0), 0);
        assert_eq!(trailing_zeroes(10000), 2499);
    }
}
