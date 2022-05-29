use std::cmp::Ordering;

#[allow(dead_code)]
static mut TARGET: i32 = 0;

#[allow(dead_code, non_snake_case)]
unsafe fn guessNumber(n: i32) -> i32 {
    let mut a = 1;
    let mut b = n;
    while a < b {
        let c = a + (b - a) / 2;
        match guess(c) {
            1 => a = c + 1,
            -1 => b = c - 1,
            _ => return c,
        }
    }
    unreachable!()
}

#[allow(dead_code)]
unsafe fn guess(num: i32) -> i32 {
    match num.cmp(&TARGET) {
        Ordering::Less => 1,
        Ordering::Greater => -1,
        Ordering::Equal => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(num: i32, target: i32) {
        unsafe {
            TARGET = target;
            assert_eq!(guessNumber(num), target);
        }
    }

    #[test]
    fn example() {
        tf(10, 6);
    }
}
