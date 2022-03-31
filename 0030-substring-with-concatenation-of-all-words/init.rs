use std::collections::HashMap;

pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
    let slen = s.len();
    let size = words[0].len();
    let wlen = size * words.len();
    if wlen > slen {
        return Vec::new();
    }
    let s = s.as_bytes();

    let mut counts: HashMap<&[u8], usize> = HashMap::new();
    for word in words.iter() {
        let k = word.as_bytes();
        if let Some(count) = counts.get_mut(k) {
            *count += 1;
        } else {
            counts.insert(k, 1);
        }
    }

    let mut rv: Vec<i32> = Vec::new();
    for i in 0..=slen - wlen {
        let mut p = i;
        let mut remains = counts.clone();
        let ok = loop {
            let k = &s[p..p + size];
            if let Some(remain) = remains.get_mut(k) {
                if *remain <= 0 {
                    break false;
                }
                *remain -= 1;
            } else {
                break false;
            }
            p += size;
            if p >= i + wlen {
                break true;
            }
        };
        if ok {
            rv.push(i as i32);
        }
    }
    return rv;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(s: &str, words: &[&str], excepted: Vec<i32>) {
        let vs = s.to_string();
        let vwords: Vec<String> = words.iter().map(|v| v.to_string()).collect();
        let mut output = find_substring(vs, vwords);
        output.sort();
        assert_eq!(output, excepted, "{:?}", s);
    }

    #[test]
    fn example() {
        tf("barfoothefoobarman", &["foo", "bar"], vec![0, 9]);
        tf(
            "wordgoodgoodgoodbestword",
            &["word", "good", "best", "word"],
            vec![],
        );
        tf(
            "barfoofoobarthefoobarman",
            &["foo", "bar", "the"],
            vec![6, 9, 12],
        );

        tf(
            "wordgoodgoodgoodbestword",
            &["word", "good", "best", "good"],
            vec![8],
        );

        tf("ababababab", &["ababa", "babab"], vec![0]);

        tf("a", &["a", "a"], vec![]);
        tf("aaaaa", &["a", "a", "a", "a", "a"], vec![0]);
        tf(
            String::from_iter(vec!['a'; 999]).as_str(),
            &vec!["a"; 998],
            vec![0, 1],
        );
    }
}
