pub fn discount_prices(sentence: String, discount: i32) -> String {
    let z = (100 - discount) as f64 / 100.0;
    let answer: Vec<String> = sentence
        .as_str()
        .split_whitespace()
        .map(|s| {
            if &s[..1] == "$" {
                if let Ok(v) = &s[1..].parse::<f64>() {
                    format!("${:.2}", v * z)
                } else {
                    s.to_string()
                }
            } else {
                s.to_string()
            }
        })
        .collect();
    answer.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(input: &str, discount: i32, expected: &str) {
        let output = discount_prices(input.to_string(), discount);
        assert_eq!(output, expected, "{:?}", input);
    }

    #[test]
    fn example() {
        tf(
            "there are $1 $2 and 5$ candies in the shop",
            50,
            "there are $0.50 $1.00 and 5$ candies in the shop",
        );

        tf(
            "1 2 $3 4 $5 $6 7 8$ $9 $10$",
            100,
            "1 2 $0.00 4 $0.00 $0.00 7 8$ $0.00 $10$",
        );

        tf("$7383692 5q $5870426", 64, "$2658129.12 5q $2113353.36")
    }
}
