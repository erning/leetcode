pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
    fn is_exist(
        x: i32,
        y: i32,
        board: &Vec<Vec<char>>,
        word: &str,
        visited: &mut Vec<Vec<bool>>,
    ) -> bool {
        const DIRS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
        if word.is_empty() {
            return true;
        }
        if visited[y as usize][x as usize] {
            return false;
        }
        let ch = word.chars().next().unwrap();
        if board[y as usize][x as usize] != ch {
            return false;
        }
        if word.len() <= 1 {
            return true;
        }
        visited[y as usize][x as usize] = true;
        for (dx, dy) in DIRS {
            let nx = x + dx;
            let ny = y + dy;
            if nx >= 0 && nx < board[0].len() as i32 && ny >= 0 && ny < board.len() as i32 {
                let v = is_exist(nx, ny, board, &word[1..], visited);
                if v {
                    return true;
                }
            }
        }
        visited[y as usize][x as usize] = false;
        false
    }

    let mut visited: Vec<Vec<bool>> = vec![vec![false; board[0].len()]; board.len()];
    for y in 0..board.len() {
        for x in 0..board[0].len() {
            let v = is_exist(x as i32, y as i32, &board, &word, &mut visited);
            if v {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            exist(
                vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E'],
                ],
                "ABCCED".to_string(),
            ),
            true
        );
        assert_eq!(
            exist(
                vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E']
                ],
                "SEE".to_string()
            ),
            true
        );
        assert_eq!(
            exist(
                vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E']
                ],
                "ABCB".to_string()
            ),
            false
        );

        assert_eq!(exist(vec![vec!['a']], "a".to_string()), true);
    }
}
