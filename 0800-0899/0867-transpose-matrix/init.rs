pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let m = matrix.len();
    let n = matrix[0].len();
    let mut answer = vec![vec![0; m]; n];

    for (y, row) in matrix.into_iter().enumerate() {
        for (x, v) in row.into_iter().enumerate() {
            answer[x][y] = v;
        }
    }

    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(input: &[&[i32]], expected: &[&[i32]]) {
        let matrix: Vec<Vec<i32>> = input.iter().map(|v| v.to_vec()).collect();
        let output = transpose(matrix);
        assert_eq!(output, expected, "{:?}", input);
    }

    #[test]
    fn example() {
        tf(
            &[&[1, 2, 3], &[4, 5, 6], &[7, 8, 9]],
            &[&[1, 4, 7], &[2, 5, 8], &[3, 6, 9]],
        );

        tf(&[&[1, 2, 3], &[4, 5, 6]], &[&[1, 4], &[2, 5], &[3, 6]])
    }
}
