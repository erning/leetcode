pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
    let a = left as u32;
    let b = right as u32;
    let mut i = 32;
    while i > 0 {
        i -= 1;
        let n = 1u32 << i;
        if a & n < b & n {
            break;
        }
    }
    let n = 0xffffffffu32 << i;
    (a & n) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(range_bitwise_and(4, 7), 4);
        assert_eq!(range_bitwise_and(0, 0), 0);
        assert_eq!(range_bitwise_and(1, 2147483647), 0);

        assert_eq!(range_bitwise_and(100, 111), 96);

        assert_eq!(range_bitwise_and(1, 1), 1);
    }
}
