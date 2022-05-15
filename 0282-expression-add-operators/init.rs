pub fn add_operators(num: String, target: i32) -> Vec<String> {
    fn f(
        i: usize,
        exp: String,
        prev: i64,
        expval: i64,
        num: &str,
        target: i64,
        answer: &mut Vec<String>,
    ) {
        if i == num.len() {
            if expval == target {
                answer.push(exp);
            }
            return;
        }
        for j in i..num.len() {
            let cur: i64 = num[i..j + 1].parse().unwrap();
            if &num[i..i + 1] == "0" && i < j {
                return;
            }
            if i == 0 {
                f(j + 1, cur.to_string(), cur, cur, num, target, answer);
            } else {
                let fmt = |c| format!("{}{}{}", &exp, c, cur);
                f(j + 1, fmt('+'), cur, expval + cur, num, target, answer);
                f(j + 1, fmt('-'), -cur, expval - cur, num, target, answer);
                f(
                    j + 1,
                    fmt('*'),
                    prev * cur,
                    expval - prev + prev * cur,
                    num,
                    target,
                    answer,
                );
            }
        }
    }

    let mut answer: Vec<String> = Vec::new();
    f(0, String::new(), 0, 0, &num, target as i64, &mut answer);
    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(input: &str, target: i32, expected: &[&str]) {
        let mut output = add_operators(input.to_string(), target);
        output.sort_unstable();
        assert_eq!(output, expected);
    }

    #[test]
    fn example() {
        tf("123", 6, &["1*2*3", "1+2+3"]);
        tf("232", 8, &["2*3+2", "2+3*2"]);
        tf("3456237490", 9191, &[]);

        tf("105", 5, &["1*0+5", "10-5"]);
    }
}
