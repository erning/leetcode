pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
    let mut rows = vec![false; matrix.len()];
    let mut cols = vec![false; matrix[0].len()];
    for y in 0..matrix.len() {
        for (x, &v) in matrix[y].iter().enumerate() {
            if v == 0 {
                cols[x] = true;
                rows[y] = true;
            }
        }
    }
    for (y, _) in rows.iter().enumerate().filter(|&(_, &v)| v) {
        for v in matrix[y].iter_mut() {
            *v = 0;
        }
    }
    for (x, _) in cols.iter().enumerate().filter(|&(_, &v)| v) {
        for y in 0..matrix.len() {
            matrix[y][x] = 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(input: Vec<Vec<i32>>, expected: Vec<Vec<i32>>) {
        let mut matrix = input.clone();
        set_zeroes(&mut matrix);
        assert_eq!(matrix, expected, "input: {:?}", input);
    }

    #[test]
    fn example() {
        tf(
            vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]],
            vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]],
        );
        tf(
            vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]],
            vec![vec![0, 0, 0, 0], vec![0, 4, 5, 0], vec![0, 3, 1, 0]],
        );
    }
}
