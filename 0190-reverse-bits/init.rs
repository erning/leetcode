pub fn reverse_bits(x: u32) -> u32 {
    let mut y = 0;
    for i in 0..32 {
        if x & (1 << i) > 0 {
            y |= 1 << (31 - i)
        }
    }
    y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(reverse_bits(0b00000010100101000001111010011100), 964176192);
        assert_eq!(reverse_bits(0b11111111111111111111111111111101), 3221225471);
    }
}
