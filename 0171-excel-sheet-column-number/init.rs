pub fn title_to_number(column_title: String) -> i32 {
    let mut n = 0;
    for c in column_title.as_bytes().into_iter() {
        n = n * 26 + (c - 0x40) as i32;
    }
    n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(title_to_number("A".to_string()), 1);
        assert_eq!(title_to_number("AB".to_string()), 28);
        assert_eq!(title_to_number("ZY".to_string()), 701);
    }
}
