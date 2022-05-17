pub fn is_happy(n: i32) -> bool {
    let mut set: Vec<bool> = vec![false; 10 * 9 * 9];
    let mut a = n;
    loop {
        let mut c = 0;
        while a > 0 {
            let b = a % 10;
            a /= 10;
            c += b * b;
        }
        a = c;
        if a == 1 {
            break true;
        }
        if set[a as usize] {
            break false;
        }
        set[a as usize] = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(is_happy(19), true);
        assert_eq!(is_happy(2), false);
        assert_eq!(is_happy(1), true);
        assert_eq!(is_happy(i32::MAX), false);
    }
}
