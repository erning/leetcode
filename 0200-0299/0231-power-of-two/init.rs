pub fn is_power_of_two(n: i32) -> bool {
    n > 0 && n.count_ones() == 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(is_power_of_two(1), true);
        assert_eq!(is_power_of_two(16), true);
        assert_eq!(is_power_of_two(3), false);

        assert_eq!(is_power_of_two(-16), false);
        assert_eq!(is_power_of_two(i32::MIN), false);
    }
}
