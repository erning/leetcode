use std::collections::VecDeque;

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

    let mut word_list: Vec<&str> = word_list.iter().map(|s| s.as_str()).collect();
    let mut queue = VecDeque::new();
    {
        let mut v = Vec::new();
        for (i, &next) in word_list.iter().enumerate() {
            if is_next(&begin_word, next) {
                queue.push_back((1, next));
                v.push(i);
            }
        }
        v.into_iter().rev().for_each(|i| {
            word_list.swap_remove(i);
        });
    }
    while let Some((c, word)) = queue.pop_front() {
        if word == &end_word {
            return c + 1;
        }
        let mut v = Vec::new();
        for (i, &next) in word_list.iter().enumerate() {
            if is_next(word, next) {
                queue.push_back((c + 1, next));
                v.push(i);
            }
        }
        v.into_iter().rev().for_each(|i| {
            word_list.swap_remove(i);
        });
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
