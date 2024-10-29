pub fn fib(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }
    let (mut a, mut b) = (0, 1);
    let mut m = n - 1;
    while m > 0 {
        let c = a + b;
        a = b;
        b = c;
        m -= 1;
    }
    b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(fib(0), 0);
        assert_eq!(fib(1), 1);
        assert_eq!(fib(2), 1);
        assert_eq!(fib(3), 2);
        assert_eq!(fib(4), 3);
        assert_eq!(fib(5), 5);
        assert_eq!(fib(30), 832040);
    }
}
