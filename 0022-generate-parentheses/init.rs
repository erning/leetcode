pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let n = n as usize;
    let mut prev: Vec<Vec<usize>> = vec![vec![0; n]];
    for i in 1..n {
        let mut next: Vec<Vec<usize>> = Vec::new();
        for j in i..=i << 1 {
            let mut curr = prev.to_vec();
            for v in curr.iter_mut() {
                if v[i - 1] >= j {
                    v.clear();
                } else {
                    v[i] = j;
                }
            }
            curr.retain(|v| !v.is_empty());
            next.append(&mut curr);
        }
        prev = next;
    }

    let mut rv: Vec<String> = Vec::new();
    for v in prev.iter() {
        let mut chars: Vec<char> = vec![')'; n * 2];
        for i in v.iter() {
            chars[*i] = '(';
        }
        rv.push(chars.into_iter().collect());
    }
    rv
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(input: i32, expected: Vec<&str>) {
        let mut output = generate_parenthesis(input);
        output.sort_unstable();
        assert_eq!(output, expected);
    }
    #[test]
    fn example() {
        tf(1, vec!["()"]);
        tf(2, vec!["(())", "()()"]);
        tf(3, vec!["((()))", "(()())", "(())()", "()(())", "()()()"]);
    }
}
