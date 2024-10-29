use std::cmp::Ordering;

pub fn alphabet_board_path(target: String) -> String {
    let mut answer: Vec<u8> = Vec::new();
    let mut curr = (0, 0);
    for ch in target.as_bytes().iter() {
        let next = ((ch - b'a') as usize / 5, (ch - b'a') as usize % 5);
        if next != curr {
            if curr.0 == 5 {
                answer.push(b'U');
                curr.0 -= 1;
            }
            match next.1.cmp(&curr.1) {
                Ordering::Less => answer.append(&mut vec![b'L'; curr.1 - next.1]),
                Ordering::Greater => answer.append(&mut vec![b'R'; next.1 - curr.1]),
                Ordering::Equal => {}
            }
            match next.0.cmp(&curr.0) {
                Ordering::Less => answer.append(&mut vec![b'U'; curr.0 - next.0]),
                Ordering::Greater => answer.append(&mut vec![b'D'; next.0 - curr.0]),
                Ordering::Equal => {}
            }
        }
        answer.push(b'!');
        curr = next;
    }
    String::from_utf8(answer).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    // fn normalize(s: &str) -> String {
    //     s.split('!')
    //         .map(|v| {
    //             let mut s: Vec<char> = v.chars().collect();
    //             s.sort_unstable();
    //             s.into_iter().collect::<String>()
    //         })
    //         .collect::<Vec<String>>()
    //         .join("!")
    // }

    fn tf(input: &str, expected: &str) {
        let output = alphabet_board_path(input.to_string());
        // let output = normalize(output.as_str());
        // let expected = normalize(expected);
        assert_eq!(output, expected, "{:?}", input);
    }

    #[test]
    fn example() {
        tf("leet", "RDD!RRRUU!!DDD!");
        tf("code", "RR!RRDD!LUU!R!");

        tf("zdz", "DDDDD!URRRUUUU!LLLDDDDD!");
    }
}
