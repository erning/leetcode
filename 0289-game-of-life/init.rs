#[allow(clippy::ptr_arg)]
pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
    let m = board.len();
    let n = board[0].len();
    let neighbors = |x: usize, y: usize| -> i32 {
        [
            (0, -1),
            (1, -1),
            (1, 0),
            (1, 1),
            (0, 1),
            (-1, 1),
            (-1, 0),
            (-1, -1),
        ]
        .into_iter()
        .map(|(dx, dy)| (x as i32 + dx, y as i32 + dy))
        .filter(|&(x, y)| x >= 0 && x < n as i32 && y >= 0 && y < m as i32)
        .map(|(x, y)| (x as usize, y as usize))
        .filter(|&(x, y)| board[y][x] == 1)
        .count() as i32
    };
    let mut nexts: Vec<(usize, usize, i32)> = Vec::new();
    for (y, row) in board.iter().enumerate() {
        for (x, &v) in row.iter().enumerate() {
            let c = neighbors(x, y);
            match v {
                1 if !(2..=3).contains(&c) => nexts.push((x, y, 0)),
                0 if c == 3 => nexts.push((x, y, 1)),
                _ => {}
            }
        }
    }
    for (x, y, v) in nexts {
        board[y][x] = v;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(input: &[&[i32]], expected: &[&[i32]]) {
        let mut board: Vec<Vec<i32>> = input.into_iter().map(|v| v.to_vec()).collect();
        game_of_life(&mut board);
        assert_eq!(board, expected, "{:?}", input);
    }

    #[test]
    fn example() {
        tf(
            &[&[0, 1, 0], &[0, 0, 1], &[1, 1, 1], &[0, 0, 0]],
            &[&[0, 0, 0], &[1, 0, 1], &[0, 1, 1], &[0, 1, 0]],
        );
        tf(&[&[1, 1], &[1, 0]], &[&[1, 1], &[1, 1]])
    }
}
