use std::collections::HashMap;
use std::collections::HashSet;

pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
    #[derive(Debug)]
    struct TrieNode {
        ch: char,
        children: HashMap<char, TrieNode>,
        word: Option<String>,
    }

    let mut root = TrieNode {
        ch: '-',
        children: HashMap::new(),
        word: None,
    };
    for word in words.into_iter() {
        let mut parent = &mut root;
        for ch in word.chars().take(word.len() - 1) {
            parent.children.entry(ch).or_insert_with(|| TrieNode {
                ch,
                children: HashMap::new(),
                word: None,
            });
            // if !parent.children.contains_key(&ch) {
            //     let node = TrieNode {
            //         ch,
            //         children: HashMap::new(),
            //         word: None,
            //     };
            //     parent.children.insert(ch, node);
            // }
            parent = parent.children.get_mut(&ch).unwrap()
        }
        let ch = word.chars().last().unwrap();
        parent.children.entry(ch).or_insert_with(|| TrieNode {
            ch,
            children: HashMap::new(),
            word: None,
        });
        parent = parent.children.get_mut(&ch).unwrap();
        parent.word = Some(word);
    }

    fn dfs(
        x: i32,
        y: i32,
        board: &Vec<Vec<char>>,
        trie: &TrieNode,
        visited: &mut HashSet<(i32, i32)>,
        answer: &mut HashSet<String>,
    ) {
        if x < 0 || x >= board[0].len() as i32 || y < 0 || y >= board.len() as i32 {
            return;
        }
        let ch = board[y as usize][x as usize];
        if ch != trie.ch {
            return;
        }
        let k = (x, y);
        if visited.contains(&k) {
            return;
        }
        visited.insert(k);
        if let Some(word) = &trie.word {
            answer.insert(word.to_string());
        }
        for (_, next) in trie.children.iter() {
            dfs(x, y - 1, board, next, visited, answer);
            dfs(x + 1, y, board, next, visited, answer);
            dfs(x, y + 1, board, next, visited, answer);
            dfs(x - 1, y, board, next, visited, answer);
        }
        visited.remove(&k);
    }

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut answer: HashSet<String> = HashSet::new();
    for y in 0..board.len() {
        for x in 0..board[0].len() {
            for (_, trie) in root.children.iter() {
                dfs(x as i32, y as i32, &board, trie, &mut visited, &mut answer);
            }
        }
    }
    answer.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(board: &[&[char]], words: &[&str], expected: &[&str]) {
        let b: Vec<Vec<char>> = board.iter().map(|v| v.to_vec()).collect();
        let w: Vec<String> = words.iter().map(|v| v.to_string()).collect();
        let mut output = find_words(b, w);
        output.sort_unstable();
        assert_eq!(output, expected, "{:?}", (board, words));
    }
    #[test]
    fn example() {
        tf(
            &[
                &['o', 'a', 'a', 'n'],
                &['e', 't', 'a', 'e'],
                &['i', 'h', 'k', 'r'],
                &['i', 'f', 'l', 'v'],
            ],
            &["oath", "pea", "eat", "rain"],
            &["eat", "oath"],
        );
        tf(&[&['a', 'b'], &['c', 'd']], &["abcb"], &[]);

        tf(
            &[
                &['o', 'a', 'b', 'n'],
                &['o', 't', 'a', 'e'],
                &['a', 'h', 'k', 'r'],
                &['a', 'f', 'l', 'v'],
            ],
            &["oa", "oaa"],
            &["oa", "oaa"],
        );
    }
}
