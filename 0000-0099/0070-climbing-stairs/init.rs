pub fn climb_stairs(n: i32) -> i32 {
    if n == 1 {
        return 1;
    }
    if n == 2 {
        return 2;
    }
    let mut a = 1;
    let mut b = 2;
    for _ in 2..n {
        (a, b) = (b, a + b)
    }
    b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(climb_stairs(2), 2);
        assert_eq!(climb_stairs(3), 3);
        assert_eq!(climb_stairs(10), 89);
    }
}
