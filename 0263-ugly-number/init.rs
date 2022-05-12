pub fn is_ugly(n: i32) -> bool {
    if n <= 0 {
        return false;
    }
    let mut n = n;
    let mut m = 2;
    while n >= m {
        if n % m == 0 {
            n /= m;
        } else {
            m += 1;
            if m > 5 {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(is_ugly(6), true);
        assert_eq!(is_ugly(1), true);
        assert_eq!(is_ugly(14), false);
    }
}
