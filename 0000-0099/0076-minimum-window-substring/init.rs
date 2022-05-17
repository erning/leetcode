pub fn min_window(s: String, t: String) -> String {
    if s.len() < t.len() {
        return String::new();
    }

    let s = s.as_bytes();
    let t = t.as_bytes();

    let mut map_t = [0; 256];
    for ch in t.iter() {
        map_t[*ch as usize] += 1;
    }
    let map_v: Vec<usize> = map_t
        .iter()
        .enumerate()
        .filter(|&(_, &v)| v > 0)
        .map(&|(i, _)| i)
        .collect();

    let mut a = 0;
    let mut b = t.len();
    let mut min_a = 0;
    let mut min_b = 0;
    let mut min_c = usize::MAX;

    let mut map = [0; 256];
    for ch in &s[a..b] {
        map[*ch as usize] += 1;
    }

    loop {
        let mut is_satisfy = true;
        for &ch in map_v.iter() {
            if map[ch] < map_t[ch] {
                is_satisfy = false;
                break;
            }
        }
        if is_satisfy {
            let c = b - a;
            if c < min_c {
                min_a = a;
                min_b = b;
                min_c = c;
            }
            let ch = s[a];
            map[ch as usize] -= 1;
            a += 1;
            if a + t.len() <= b {
                continue;
            }
        }

        b += 1;
        if b > s.len() {
            break;
        }
        let ch = s[b - 1];
        map[ch as usize] += 1;
    }

    String::from_utf8(s[min_a..min_b].to_vec()).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(s: &str, t: &str, expected: &str) {
        let output = min_window(s.to_string(), t.to_string());
        assert_eq!(output, expected, "{:?}", (s, t));
    }

    #[test]
    fn example() {
        tf("ADOBECODEBANC", "ABC", "BANC");
        tf("a", "a", "a");
        tf("a", "aa", "");

        tf("a", "b", "");
    }
}
