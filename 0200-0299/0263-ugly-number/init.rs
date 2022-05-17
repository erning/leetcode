pub fn is_ugly(n: i32) -> bool {
    if n <= 0 {
        return false;
    }
    let mut n = n;
    while n % 2 == 0 {
        n /= 2;
    }
    while n % 3 == 0 {
        n /= 3;
    }
    while n % 5 == 0 {
        n /= 5;
    }
    n == 1
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
