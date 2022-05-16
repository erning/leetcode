use std::cmp::Ordering;

pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let m = matrix.len();
    let n = matrix[0].len();

    let mut a = 0;
    let mut b = m - 1;
    let mut c;
    while a + 1 < b {
        c = a + (b - a) / 2;
        let v = matrix[c][0];
        match target.cmp(&v) {
            Ordering::Less => b = c,
            Ordering::Greater => a = c,
            Ordering::Equal => return true,
        }
    }
    let y = if target < matrix[b][0] { a } else { b };

    let mut a = 0;
    let mut b = n - 1;
    let mut c;
    while a + 1 < b {
        c = a + (b - a) / 2;
        let v = matrix[y][c];
        match target.cmp(&v) {
            Ordering::Less => b = c,
            Ordering::Greater => a = c,
            Ordering::Equal => return true,
        }
    }
    let x = if target < matrix[y][b] { a } else { b };

    matrix[y][x] == target
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            search_matrix(
                vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
                3,
            ),
            true
        );
        assert_eq!(
            search_matrix(
                vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
                13,
            ),
            false
        );

        assert_eq!(
            search_matrix(
                vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 50]],
                11
            ),
            true
        );

        assert_eq!(search_matrix(vec![vec![1]], 1), true);
        assert_eq!(search_matrix(vec![vec![1]], 2), false);

        assert_eq!(search_matrix(vec![vec![1, 2]], 1), true);
        assert_eq!(search_matrix(vec![vec![1, 2]], 2), true);
        assert_eq!(search_matrix(vec![vec![1, 2]], 3), false);

        assert_eq!(search_matrix(vec![vec![1], vec![2]], 1), true);
        assert_eq!(search_matrix(vec![vec![1], vec![2]], 2), true);
        assert_eq!(search_matrix(vec![vec![1], vec![2]], 3), false);
    }
}
