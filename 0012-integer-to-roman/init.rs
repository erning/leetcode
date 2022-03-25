use std::collections::HashMap;

pub fn int_to_roman(num: i32) -> String {
    let symbols: HashMap<i32, &str> = [
        (0, ""),
        (1, "I"),
        (2, "II"),
        (3, "III"),
        (4, "IV"),
        (5, "V"),
        (6, "VI"),
        (7, "VII"),
        (8, "VIII"),
        (9, "IX"),
        (10, "X"),
        (20, "XX"),
        (30, "XXX"),
        (40, "XL"),
        (50, "L"),
        (60, "LX"),
        (70, "LXX"),
        (80, "LXXX"),
        (90, "XC"),
        (100, "C"),
        (200, "CC"),
        (300, "CCC"),
        (400, "CD"),
        (500, "D"),
        (600, "DC"),
        (700, "DCC"),
        (800, "DCCC"),
        (900, "CM"),
        (1000, "M"),
        (2000, "MM"),
        (3000, "MMM"),
    ]
    .into_iter()
    .collect();

    let mut num = num;
    let mut stack: Vec<&str> = vec![];
    let mut i = 1;
    while num > 0 {
        let n = (num % 10) * i;
        i *= 10;
        num /= 10;
        let s = symbols.get(&n).unwrap();
        stack.push(s);
    }
    stack.reverse();
    stack.join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(int_to_roman(3), "III");
        assert_eq!(int_to_roman(58), "LVIII");
        assert_eq!(int_to_roman(1994), "MCMXCIV");
        assert_eq!(int_to_roman(10), "X");

        assert_eq!(int_to_roman(3999), "MMMCMXCIX");
        assert_eq!(int_to_roman(1000), "M");
    }
}
