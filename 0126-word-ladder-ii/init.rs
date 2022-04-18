pub fn find_ladders(
    begin_word: String,
    end_word: String,
    word_list: Vec<String>,
) -> Vec<Vec<String>> {
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
    words.push(begin_word.as_str());
    for word in word_list.iter() {
        if word == &begin_word {
            continue;
        }
        words.push(word.as_str());
    }

    let len = words.len();
    let mut end = len;
    let mut graph = vec![vec![]; len];
    {
        for (i, a) in words.iter().enumerate() {
            if a == &end_word {
                end = i;
            }
            for (j, b) in words.iter().enumerate().skip(i) {
                if is_next(a, b) {
                    graph[i].push(j);
                    graph[j].push(i);
                }
            }
        }
    }
    if end >= len {
        return Vec::new();
    }

    let to_string_vec = |path: &Vec<usize>| -> Vec<String> {
        path.into_iter().map(|&i| words[i].to_string()).collect()
    };

    let mut paths: Vec<Vec<String>> = Vec::new();
    let mut queue: Vec<Vec<usize>> = Vec::new();
    let mut visited = vec![false; words.len()];
    let mut next_queue: Vec<Vec<usize>> = Vec::new();

    queue.push(vec![0]);
    visited[0] = true;

    while paths.is_empty() && !queue.is_empty() {
        let mut next_visited: Vec<usize> = Vec::new();
        while let Some(path) = queue.pop() {
            let &word = path.last().unwrap();
            if word == end {
                paths.push(to_string_vec(&path));
                continue;
            }
            for &next in graph[word].iter() {
                if visited[next] {
                    continue;
                }
                let mut next_path = path.clone();
                next_path.push(next);
                next_queue.push(next_path);
                next_visited.push(next);
            }
        }
        for i in next_visited.into_iter() {
            visited[i] = true;
        }
        std::mem::swap(&mut queue, &mut next_queue);
    }

    paths
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(begin_word: &str, end_word: &str, word_list: Vec<&str>, expected: Vec<Vec<&str>>) {
        let begin = begin_word.to_string();
        let end = end_word.to_string();
        let word: Vec<String> = word_list.iter().map(|&v| v.to_string()).collect();
        let mut output = find_ladders(begin, end, word);
        output.sort();
        assert_eq!(output, expected, "{:?}", (begin_word, end_word, word_list));
    }

    #[test]
    fn example() {
        tf(
            "hit",
            "cog",
            vec!["hot", "dot", "dog", "lot", "log", "cog"],
            vec![
                vec!["hit", "hot", "dot", "dog", "cog"],
                vec!["hit", "hot", "lot", "log", "cog"],
            ],
        );
        tf(
            "hit",
            "cog",
            vec!["hot", "dot", "dog", "lot", "log"],
            vec![],
        );

        tf(
            "red",
            "tax",
            vec!["ted", "tex", "red", "tax", "tad", "den", "rex", "pee"],
            vec![
                vec!["red", "rex", "tex", "tax"],
                vec!["red", "ted", "tad", "tax"],
                vec!["red", "ted", "tex", "tax"],
            ],
        );
    }
}
