pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
    let max_width = max_width as usize;
    let mut rv: Vec<String> = Vec::new();

    let mut iter = words.iter();
    let mut opt = iter.next();
    let mut row: Vec<String> = Vec::new();
    let mut c = max_width;
    while let Some(word) = opt {
        if c >= word.len() + row.len() {
            row.push(word.clone());
            c -= word.len();
            opt = iter.next();
            continue;
        }
        let split_count = row.len() - 1;
        let s = if split_count > 0 {
            let d = c / split_count;
            let m = c % split_count;
            if m == 0 {
                let sep = String::from_utf8(vec![b' '; d]).unwrap();
                row.join(sep.as_str()).to_string()
            } else {
                let mut sep = String::from_utf8(vec![b' '; d + 1]).unwrap();
                let mut s = String::with_capacity(max_width);
                s.push_str(&row[..=m].join(sep.as_str()).to_string());
                sep.pop();
                for word in &row[m + 1..] {
                    s.push_str(sep.as_str());
                    s.push_str(word);
                }
                s
            }
        } else {
            let mut s = String::with_capacity(max_width);
            s.push_str(row.join(" ").as_str());
            while s.len() < max_width {
                s.push(' ');
            }
            s
        };
        rv.push(s);
        row = Vec::new();
        c = max_width;
    }
    if !row.is_empty() {
        let mut s = String::with_capacity(max_width);
        s.push_str(row.join(" ").as_str());
        while s.len() < max_width {
            s.push(' ');
        }
        rv.push(s);
    }

    rv
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(words: Vec<&str>, max_width: i32, expected: Vec<&str>) {
        let input: Vec<String> = words.iter().map(|v| v.to_string()).collect();
        let output = full_justify(input, max_width);
        assert_eq!(output, expected, "{:?}", output);
    }

    #[test]
    fn example() {
        tf(
            vec![
                "This",
                "is",
                "an",
                "example",
                "of",
                "text",
                "justification.",
            ],
            16,
            vec!["This    is    an", "example  of text", "justification.  "],
        );

        tf(
            vec!["What", "must", "be", "acknowledgment", "shall", "be"],
            16,
            vec!["What   must   be", "acknowledgment  ", "shall be        "],
        );

        tf(
            vec![
                "Science",
                "is",
                "what",
                "we",
                "understand",
                "well",
                "enough",
                "to",
                "explain",
                "to",
                "a",
                "computer.",
                "Art",
                "is",
                "everything",
                "else",
                "we",
                "do",
            ],
            20,
            vec![
                "Science  is  what we",
                "understand      well",
                "enough to explain to",
                "a  computer.  Art is",
                "everything  else  we",
                "do                  ",
            ],
        );
    }
}
