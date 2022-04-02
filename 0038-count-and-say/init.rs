pub fn count_and_say(n: i32) -> String {
    let mut curr: Vec<u8> = vec![b'1'];
    for _ in 1..n {
        let mut next: Vec<u8> = Vec::new();
        let mut p = curr[0];
        let mut b = 1;
        for c in curr.iter().skip(1) {
            if *c == p {
                b += 1;
            } else {
                next.extend_from_slice(b.to_string().as_bytes());
                next.push(p);
                b = 1;
                p = *c;
            }
        }
        next.extend_from_slice(b.to_string().as_bytes());
        next.push(p);
        curr = next;
    }
    String::from_utf8(curr).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(count_and_say(1), "1");
        assert_eq!(count_and_say(2), "11");
        assert_eq!(count_and_say(3), "21");
        assert_eq!(count_and_say(4), "1211");
    }
}
