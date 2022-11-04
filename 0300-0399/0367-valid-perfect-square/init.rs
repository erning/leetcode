use std::cmp::Ordering;

pub fn is_perfect_square(num: i32) -> bool {
    let mut a = 1;
    let mut b = num;
    while a <= b {
        let c = a + (b - a) / 2;
        match (num / c).cmp(&c) {
            Ordering::Greater => a = c + 1,
            Ordering::Less => b = c - 1,
            Ordering::Equal => return num % c == 0,
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(is_perfect_square(16), true);
        assert_eq!(is_perfect_square(14), false);

        assert_eq!(is_perfect_square(999 * 999 + 1), false);
        assert_eq!(is_perfect_square(256 * 256), true);
        assert_eq!(is_perfect_square(i32::MAX), false);
    }
}
