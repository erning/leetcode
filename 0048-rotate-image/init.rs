pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
    let len = matrix.len();
    for x in 0..len / 2 {
        for y in 0..(len + 1) / 2 {
            let s = matrix.len() - 1;
            let t = matrix[y][x];
            matrix[y][x] = matrix[s - x][y];
            matrix[s - x][y] = matrix[s - y][s - x];
            matrix[s - y][s - x] = matrix[x][s - y];
            matrix[x][s - y] = t;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        rotate(&mut matrix);
        assert_eq!(matrix, vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]]);

        let mut matrix = vec![
            vec![11, 12, 13, 14],
            vec![15, 16, 17, 18],
            vec![19, 20, 21, 22],
            vec![23, 24, 25, 26],
        ];
        rotate(&mut matrix);
        assert_eq!(
            matrix,
            vec![
                vec![23, 19, 15, 11],
                vec![24, 20, 16, 12],
                vec![25, 21, 17, 13],
                vec![26, 22, 18, 14]
            ]
        );
    }
}
