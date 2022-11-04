pub struct NumMatrix {
    presum: Vec<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumMatrix {
    pub fn new(matrix: Vec<Vec<i32>>) -> Self {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut presum = vec![vec![0; n + 1]; m + 1];
        presum[1][1] = matrix[0][0];
        for y in 1..=m {
            for x in 1..=n {
                presum[y][x] = presum[y - 1][x] + presum[y][x - 1] - presum[y - 1][x - 1]
                    + matrix[y - 1][x - 1];
            }
        }
        NumMatrix { presum }
    }

    pub fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let x1 = col1 as usize;
        let y1 = row1 as usize;
        let x2 = col2 as usize;
        let y2 = row2 as usize;

        // a b
        // c d
        let a = self.presum[y1][x1];
        let b = self.presum[y1][x2 + 1];
        let c = self.presum[y2 + 1][x1];
        let d = self.presum[y2 + 1][x2 + 1];

        a + d - b - c
    }
}

/*
 * Your NumMatrix object will be instantiated and called as such:
 * let obj = NumMatrix::new(matrix);
 * let ret_1: i32 = obj.sum_region(row1, col1, row2, col2);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let matrix = NumMatrix::new(vec![
            vec![3, 0, 1, 4, 2],
            vec![5, 6, 3, 2, 1],
            vec![1, 2, 0, 1, 5],
            vec![4, 1, 0, 1, 7],
            vec![1, 0, 3, 0, 5],
        ]);

        assert_eq!(matrix.sum_region(2, 1, 4, 3), 8);
    }
}
