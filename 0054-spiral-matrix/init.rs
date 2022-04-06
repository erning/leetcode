pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let m = matrix.len();
    let n = matrix[0].len();
    let mut rv = Vec::with_capacity(m * n);

    let mut x1: i32 = 0;
    let mut y1: i32 = 0;
    let mut x2: i32 = n as i32 - 1;
    let mut y2: i32 = m as i32 - 1;
    let mut d = 0;

    while x1 <= x2 && y1 <= y2 {
        match d {
            0 => {
                for x in x1..=x2 {
                    rv.push(matrix[y1 as usize][x as usize]);
                }
                y1 += 1;
            }
            1 => {
                for y in y1..=y2 {
                    rv.push(matrix[y as usize][x2 as usize]);
                }
                x2 -= 1;
            }
            2 => {
                for x in (x1..=x2).rev() {
                    rv.push(matrix[y2 as usize][x as usize]);
                }
                y2 -= 1;
            }
            3 => {
                for y in (y1..=y2).rev() {
                    rv.push(matrix[y as usize][x1 as usize]);
                }
                x1 += 1;
            }
            _ => unimplemented!(),
        }
        d = (d + 1) % 4;
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
