pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
    #[inline]
    fn is_next(a: &str, b: &str) -> bool {
        let a = a.as_bytes();
        let b = b.as_bytes();
        let mut diff = 0;
        for i in 0..a.len() {
            if a[i] != b[i] {
                diff += 1;
                if diff > 1 {
                    return false;
                }
            }
        }
        diff == 1
    }

    let mut words: Vec<&str> = Vec::new();
    for word in word_list.iter() {
        if word == &begin_word {
            continue;
        }
        words.push(word.as_str());
    }

    let mut queue = vec![begin_word.as_str()];
    let mut step = 0;
    while !queue.is_empty() {
        step += 1;
        let mut new_queue: Vec<&str> = Vec::new();
        while let Some(word) = queue.pop() {
            if word == end_word.as_str() {
                return step;
            }
            let mut nexts: Vec<&str> = Vec::new();
            let mut removes: Vec<usize> = Vec::new();
            for (i, &next) in words.iter().enumerate() {
                if is_next(word, next) {
                    nexts.push(next);
                    removes.push(i);
                }
            }
            for i in removes.into_iter().rev() {
                words.swap_remove(i);
            }
            new_queue.append(&mut nexts);
        }
        std::mem::swap(&mut queue, &mut new_queue);
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(begin_word: &str, end_word: &str, word_list: Vec<&str>, expected: i32) {
        let begin = begin_word.to_string();
        let end = end_word.to_string();
        let word: Vec<String> = word_list.iter().map(|&v| v.to_string()).collect();
        let output = ladder_length(begin, end, word);
        assert_eq!(output, expected, "{:?}", (begin_word, end_word, word_list));
    }

    #[test]
    fn example() {
        tf(
            "hit",
            "cog",
            vec!["hot", "dot", "dog", "lot", "log", "cog"],
            5,
        );
        tf("hit", "cog", vec!["hot", "dot", "dog", "lot", "log"], 0);

        tf("a", "c", vec!["a", "b", "c"], 2);
    }
}
