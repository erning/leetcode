pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    let mut rcb1 = [[false; 9]; 9];
    let mut rcb2 = [[false; 9]; 9];
    let mut rcb3 = [[false; 9]; 9];
    for (y, row) in board.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if let Some(b) = c.to_digit(10) {
                let b = b as usize - 1;
                let vx = &mut rcb1[x][b];
                let vy = &mut rcb2[y][b];
                if *vx || *vy {
                    return false;
                }
                let z = y / 3 * 3 + x / 3 % 3;
                let vz = &mut rcb3[z][b];
                if *vz {
                    return false;
                }
                *vx = true;
                *vy = true;
                *vz = true;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(board: [[char; 9]; 9], expected: bool) {
        let input: Vec<Vec<char>> = board.into_iter().map(|v| v.to_vec()).collect();
        let output = is_valid_sudoku(input);
        assert_eq!(output, expected);
    }

    #[test]
    fn example() {
        tf(
            [
                ['5', '3', '.', '.', '7', '.', '.', '.', '.'],
                ['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                ['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                ['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                ['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                ['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                ['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                ['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                ['.', '.', '.', '.', '8', '.', '.', '7', '9'],
            ],
            true,
        );

        tf(
            [
                ['8', '3', '.', '.', '7', '.', '.', '.', '.'],
                ['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                ['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                ['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                ['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                ['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                ['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                ['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                ['.', '.', '.', '.', '8', '.', '.', '7', '9'],
            ],
            false,
        );
    }
}
