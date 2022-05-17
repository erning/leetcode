#[allow(non_snake_case)]
pub fn hammingWeight(n: u32) -> i32 {
    let mut n = n;
    let mut count = 0;
    while n > 0 {
        if n & 1 > 0 {
            count += 1;
        }
        n >>= 1;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(hammingWeight(0b00000000000000000000000000001011), 3);
        assert_eq!(hammingWeight(0b00000000000000000000000010000000), 1);
        assert_eq!(hammingWeight(0b11111111111111111111111111111101), 31);
    }
}
