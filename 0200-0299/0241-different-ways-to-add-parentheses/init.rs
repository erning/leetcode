use std::collections::HashMap;

pub fn diff_ways_to_compute(expression: String) -> Vec<i32> {
    fn f<'a>(expr: &'a str, memo: &mut HashMap<&'a str, Vec<i32>>) -> Vec<i32> {
        // match expr.parse::<i32>() {
        //     Ok(v) => {
        //         return vec![v];
        //     }
        //     _ => {}
        // }
        if let Ok(v) = expr.parse::<i32>() {
            return vec![v];
        }
        if let Some(rv) = memo.get(expr) {
            return rv.clone();
        }
        let mut rv = Vec::new();
        for (i, ch) in expr.char_indices() {
            // if ch >= '0' && ch <= '9' {
            // if ('0'..='9').contains(&ch) {
            if ch.is_ascii_digit() {
                continue;
            }
            for a in f(&expr[..i], memo).into_iter() {
                for b in f(&expr[i + 1..], memo).into_iter() {
                    let v = match ch {
                        '+' => a + b,
                        '-' => a - b,
                        '*' => a * b,
                        _ => {
                            unimplemented!()
                        }
                    };
                    rv.push(v);
                }
            }
        }
        memo.insert(expr, rv.clone());
        rv
    }

    let mut memo: HashMap<&str, Vec<i32>> = HashMap::new();
    f(&expression, &mut memo)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(input: &str, expected: &[i32]) {
        let mut output = diff_ways_to_compute(input.to_string());
        output.sort_unstable();
        assert_eq!(output, expected, "{:?}", input);
    }

    #[test]
    fn example() {
        tf("2-1-1", &[0, 2]);
        tf("2*3-4*5", &[-34, -14, -10, -10, 10]);
    }
}
