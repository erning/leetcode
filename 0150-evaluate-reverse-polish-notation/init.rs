pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut stack: Vec<i32> = Vec::new();

    macro_rules! eval {
        ($op:tt) => {
            let b: i32 = stack.pop().unwrap();
            let a: i32 = stack.pop().unwrap();
            let c = a $op b;
            stack.push(c);

        };
    }

    for v in tokens.into_iter() {
        match v.as_str() {
            "+" => {
                eval!(+);
            }
            "-" => {
                eval!(-);
            }
            "*" => {
                eval!(*);
            }
            "/" => {
                eval!(/);
            }
            _ => stack.push(v.parse::<i32>().unwrap()),
        }
    }
    stack.pop().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(input: Vec<&str>, excepted: i32) {
        let tokens: Vec<String> = input.iter().map(|v| v.to_string()).collect();
        assert_eq!(eval_rpn(tokens), excepted, "{:?}", input);
    }
    #[test]
    fn example() {
        tf(vec!["2", "1", "+", "3", "*"], 9);
        tf(vec!["4", "13", "5", "/", "+"], 6);
        tf(
            vec![
                "10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+",
            ],
            22,
        );
    }
}
