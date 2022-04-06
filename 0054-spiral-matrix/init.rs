pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let m = matrix.len();
    let n = matrix[0].len();
    let mut rv = Vec::with_capacity(m * n);

    let mut x1 = 0;
    let mut y1 = 0;
    let mut x2 = n;
    let mut y2 = m;

    loop {
        rv.extend(matrix[y1][x1..x2].iter());
        if rv.len() >= m * n {
            break;
        }
        y1 += 1;

        for row in matrix[y1..y2].iter() {
            rv.push(row[x2 - 1]);
        }
        if rv.len() >= m * n {
            break;
        }
        x2 -= 1;

        rv.extend(matrix[y2 - 1][x1..x2].iter().rev());
        if rv.len() >= m * n {
            break;
        }
        y2 -= 1;

        for row in matrix[y1..y2].iter().rev() {
            rv.push(row[x1]);
        }
        if rv.len() >= m * n {
            break;
        }
        x1 += 1;
    }

    rv
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            vec![1, 2, 3, 6, 9, 8, 7, 4, 5]
        );
        assert_eq!(
            spiral_order(vec![
                vec![1, 2, 3, 4],
                vec![5, 6, 7, 8],
                vec![9, 10, 11, 12]
            ]),
            vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]
        );
        assert_eq!(spiral_order(vec![vec![3], vec![2]]), vec![3, 2]);
        assert_eq!(spiral_order(vec![vec![3, 2]]), vec![3, 2]);
    }
}
