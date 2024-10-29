pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
    let target = {
        const A: u32 = b'a' as u32;
        let c = (u32::from(target) - A + 1) % 26 + A;
        char::from_u32(c).unwrap()
    };
    let v = letters.binary_search(&target);
    match v {
        Err(i) if i >= letters.len() => letters[0],
        Ok(i) | Err(i) => letters[i],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(input: &str, target: char, expected: char) {
        let letters: Vec<char> = input.chars().collect();
        let output = next_greatest_letter(letters, target);
        assert_eq!(output, expected, "{:?}", (input, target));
    }

    #[test]
    fn example() {
        tf("cfg", 'a', 'c');
        tf("cfg", 'c', 'f');
        tf("cfg", 'd', 'f');
        tf("cfg", 'z', 'c');

        tf("ab", 'z', 'a');
    }
}
