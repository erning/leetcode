pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
    fn f(oz: &[(usize, usize)], a: usize, b: usize, c: usize, n: usize, m: usize, max: &mut usize) {
        if a > n || b > m {
            return;
        }
        if c > *max {
            *max = c;
        }
        if oz.is_empty() {
            return;
        }
        for (i, (o, z)) in oz.iter().enumerate() {
            f(&oz[i + 1..], a + o, b + z, c + 1, n, m, max);
        }
    }

    let oz: Vec<(_, _)> = strs
        .iter()
        .map(|s| {
            let o: usize = s.chars().filter(|&c| c == '1').count();
            let z = s.len() - o;
            (o, z)
        })
        .collect();
    let mut max = 0;
    f(&oz, 0, 0, 0, n as usize, m as usize, &mut max);
    max as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(input: &[&str], m: i32, n: i32, expected: i32) {
        let strs: Vec<_> = input.iter().map(|v| v.to_string()).collect();
        let output = find_max_form(strs, m, n);
        assert_eq!(output, expected, "{:?}", input);
    }

    #[test]
    fn example() {
        tf(&["10", "0001", "111001", "1", "0"], 5, 3, 4);
        tf(&["10", "0", "1"], 1, 1, 2);

        tf(
            &[
                "11", "11", "0", "0", "10", "1", "1", "0", "11", "1", "0", "111", "11111000", "0",
                "11", "000", "1", "1", "0", "00", "1", "101", "001", "000", "0", "00", "0011", "0",
                "10000",
            ],
            90,
            66,
            1,
        );
    }
}

