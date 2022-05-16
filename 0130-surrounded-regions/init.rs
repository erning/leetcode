pub fn solve(board: &mut Vec<Vec<char>>) {
    const DIR: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let m = board.len();
    let n = board[0].len();

    fn search(x: usize, y: usize, board: &mut Vec<Vec<char>>) {
        if board[y][x] != 'O' {
            return;
        }
        board[y][x] = 'o';
        for (dx, dy) in DIR.into_iter() {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx < 0 || nx >= board[0].len() as i32 || ny < 0 || ny >= board.len() as i32 {
                continue;
            }
            search(nx as usize, ny as usize, board);
        }
    }

    for i in 0..n {
        search(i, 0, board);
        search(i, m - 1, board);
    }
    for i in 1..m - 1 {
        search(0, i, board);
        search(n - 1, i, board);
    }
    for row in board {
        for c in row.iter_mut() {
            if *c == 'o' {
                *c = 'O';
            } else if *c == 'O' {
                *c = 'X';
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(input: Vec<Vec<char>>, expected: Vec<Vec<char>>) {
        let mut board = input.clone();
        solve(&mut board);
        assert_eq!(board, expected);
    }
    #[test]
    fn example() {
        tf(
            vec![
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'O', 'O', 'X'],
                vec!['X', 'X', 'O', 'X'],
                vec!['X', 'O', 'X', 'X'],
            ],
            vec![
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'O', 'X', 'X'],
            ],
        );
        tf(vec![vec!['X']], vec![vec!['X']]);

        tf(
            vec![
                vec!['O', 'X', 'O', 'O', 'O', 'O', 'O', 'O', 'O'],
                vec!['O', 'O', 'O', 'X', 'O', 'O', 'O', 'O', 'X'],
                vec!['O', 'X', 'O', 'X', 'O', 'O', 'O', 'O', 'X'],
                vec!['O', 'O', 'O', 'O', 'X', 'O', 'O', 'O', 'O'],
                vec!['X', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'X'],
                vec!['X', 'X', 'O', 'O', 'X', 'O', 'X', 'O', 'X'],
                vec!['O', 'O', 'O', 'X', 'O', 'O', 'O', 'O', 'O'],
                vec!['O', 'O', 'O', 'X', 'O', 'O', 'O', 'O', 'O'],
                vec!['O', 'O', 'O', 'O', 'O', 'X', 'X', 'O', 'O'],
            ],
            vec![
                vec!['O', 'X', 'O', 'O', 'O', 'O', 'O', 'O', 'O'],
                vec!['O', 'O', 'O', 'X', 'O', 'O', 'O', 'O', 'X'],
                vec!['O', 'X', 'O', 'X', 'O', 'O', 'O', 'O', 'X'],
                vec!['O', 'O', 'O', 'O', 'X', 'O', 'O', 'O', 'O'],
                vec!['X', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'X'],
                vec!['X', 'X', 'O', 'O', 'X', 'O', 'X', 'O', 'X'],
                vec!['O', 'O', 'O', 'X', 'O', 'O', 'O', 'O', 'O'],
                vec!['O', 'O', 'O', 'X', 'O', 'O', 'O', 'O', 'O'],
                vec!['O', 'O', 'O', 'O', 'O', 'X', 'X', 'O', 'O'],
            ],
        );
    }
}
