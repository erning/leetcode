pub fn compare_version(version1: String, version2: String) -> i32 {
    let mut iter1 = version1.split('.');
    let mut iter2 = version2.split('.');
    let mut opt1 = iter1.next();
    let mut opt2 = iter2.next();
    while opt1.is_some() || opt2.is_some() {
        let v1: i32 = opt1.unwrap_or("0").parse().unwrap();
        let v2: i32 = opt2.unwrap_or("0").parse().unwrap();
        if v1 > v2 {
            return 1;
        }
        if v1 < v2 {
            return -1;
        }
        opt1 = iter1.next();
        opt2 = iter2.next();
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(v1: &str, v2: &str, expected: i32) {
        let output = compare_version(v1.to_string(), v2.to_string());
        assert_eq!(output, expected, "{:?}", (v1, v2));
    }
    #[test]
    fn example() {
        tf("1.01", "1.001", 0);
        tf("1.0", "1.0.0", 0);
        tf("0.1", "1.1", -1);

        tf("1", "1.0", 0);
        tf("1", "1.00", 0);
        tf("1", "1.0.1", -1);
    }
}
