pub fn calculate(s: String) -> i32 {
    #[derive(Debug)]
    enum Item {
        V(i32),
        O(char),
    }
    let mut rpn: Vec<Item> = Vec::new();
    let mut ops: Vec<char> = Vec::new();

    let mut iter = s.chars().peekable();
    let mut require_value = true;

    while let Some(ch) = iter.next() {
        match ch {
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                let mut v = ch as i32 - b'0' as i32;
                while let Some(&ch) = iter.peek() {
                    if ch < '0' || ch > '9' {
                        break;
                    }
                    v = v * 10 + (ch as i32 - b'0' as i32);
                    iter.next();
                }
                rpn.push(Item::V(v));
                require_value = false;
            }
            '-' if require_value => {
                while let Some(&o) = ops.last() {
                    match o {
                        '_' => {
                            rpn.push(Item::O(o));
                            ops.pop();
                        }
                        _ => break,
                    }
                }
                ops.push('_');
                rpn.push(Item::V(0));
                require_value = true;
            }
            '+' | '-' => {
                while let Some(&o) = ops.last() {
                    match o {
                        '_' | '*' | '/' | '+' | '-' => {
                            rpn.push(Item::O(o));
                            ops.pop();
                        }
                        _ => break,
                    }
                }
                ops.push(ch);
                require_value = true;
            }
            '*' | '/' => {
                while let Some(&o) = ops.last() {
                    match o {
                        '_' | '+' | '-' => {
                            rpn.push(Item::O(o));
                            ops.pop();
                        }
                        _ => break,
                    }
                }
                ops.push(ch);
                require_value = true;
            }
            '(' => {
                ops.push(ch);
            }
            ')' => {
                while let Some(o) = ops.pop() {
                    if o == '(' {
                        break;
                    }
                    rpn.push(Item::O(o));
                }
            }
            _ => {}
        }
    }
    while let Some(o) = ops.pop() {
        rpn.push(Item::O(o));
    }

    let mut stack: Vec<i32> = Vec::new();
    for item in rpn.into_iter() {
        match item {
            Item::V(v) => {
                stack.push(v);
            }
            Item::O(o) => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                let c = match o {
                    '+' => a + b,
                    '-' | '_' => a - b,
                    '*' => a * b,
                    '/' => a / b,
                    _ => {
                        unimplemented!()
                    }
                };
                stack.push(c);
            }
        }
    }
    stack.pop().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(input: &str, expected: i32) {
        let output = calculate(input.to_string());
        assert_eq!(output, expected, "{:?}", input);
    }
    #[test]
    fn example() {
        tf("1 + 1", 2);
        tf("2-1 + 2 ", 3);
        tf("(1+(4+5+2)-3)+(6+8)", 23);

        tf("1", 1);
        tf("-1", -1);
        tf("3 * -1", -3);
        tf("1*-1", -1);
        tf("1*(-1+2)", 1);

        tf("- (3 + (4 + 5))", -12);
    }
}
