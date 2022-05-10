pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let n = matrix[0].len();
    let column: Vec<i32> = matrix.iter().map(|v| v[n - 1]).collect();
    let mut i = matrix[0].len();
    let j = match column.binary_search(&target) {
        Ok(_) => return true,
        Err(v) => v,
    };
    for row in matrix.into_iter().skip(j) {
        i = match row[..i].binary_search(&target) {
            Ok(_) => return true,
            Err(v) => v,
        };
        if i == 0 {
            return false;
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
            search_matrix(
                vec![
                    vec![1, 4, 7, 11, 15],
                    vec![2, 5, 8, 12, 19],
                    vec![3, 6, 9, 16, 22],
                    vec![10, 13, 14, 17, 24],
                    vec![18, 21, 23, 26, 30],
                ],
                5
            ),
            true
        );
        assert_eq!(
            search_matrix(
                vec![
                    vec![1, 4, 7, 11, 15],
                    vec![2, 5, 8, 12, 19],
                    vec![3, 6, 9, 16, 22],
                    vec![10, 13, 14, 17, 24],
                    vec![18, 21, 23, 26, 30],
                ],
                20
            ),
            false
        );
    }
}
