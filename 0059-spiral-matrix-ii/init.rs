pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
    let mut matrix = vec![vec![0; n as usize]; n as usize];
    let mut v = 0;
    let mut x1 = 0;
    let mut y1 = 0;
    let mut x2 = n as usize;
    let mut y2 = n as usize;

    loop {
        for i in x1..x2 {
            v += 1;
            matrix[y1][i] = v;
        }
        if v >= n * n {
            break;
        }
        y1 += 1;

        for i in y1..y2 {
            v += 1;
            matrix[i][x2 - 1] = v;
        }
        if v >= n * n {
            break;
        }
        x2 -= 1;

        for i in (x1..x2).rev() {
            v += 1;
            matrix[y2 - 1][i] = v;
        }
        if v >= n * n {
            break;
        }
        y2 -= 1;

        for i in (y1..y2).rev() {
            v += 1;
            matrix[i][x1] = v;
        }
        if v >= n * n {
            break;
        }
        x1 += 1;
    }

    matrix
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            generate_matrix(3),
            vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]]
        );
        assert_eq!(generate_matrix(1), vec![vec![1]]);
    }
}
