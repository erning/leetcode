pub fn divide(dividend: i32, divisor: i32) -> i32 {
    let is_negative = i32::signum(dividend) != i32::signum(divisor);

    let divisor = i64::abs(divisor as i64);
    let mut dividend = i64::abs(dividend as i64);
    let mut i: i64 = 0;

    while dividend >= divisor {
        let mut n = 0;
        while dividend >= divisor << n {
            n += 1;
        }
        n -= 1;
        dividend -= divisor << n;
        i += 1 << n;
    }

    if is_negative {
        i64::max(-i, i32::MIN as i64) as i32
    } else {
        i64::min(i, i32::MAX as i64) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(divide(10, 3), 3);
        assert_eq!(divide(7, -3), -2);

        assert_eq!(divide(1, 1), 1);
        assert_eq!(divide(9, 3), 3);
        assert_eq!(divide(12, 3), 4);

        assert_eq!(divide(-2147483648, -1), 2147483647);
        assert_eq!(divide(-2147483648, 2), -1073741824);
    }
}
