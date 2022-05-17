pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut grid = grid;
    for x in 1..n {
        grid[0][x] += grid[0][x - 1];
    }
    for y in 1..m {
        grid[y][0] += grid[y - 1][0];
    }
    for y in 1..m {
        for x in 1..n {
            grid[y][x] += i32::min(grid[y][x - 1], grid[y - 1][x]);
        }
    }
    grid[m - 1][n - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            min_path_sum(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]]),
            7
        );
        assert_eq!(min_path_sum(vec![vec![1, 2, 3], vec![4, 5, 6]]), 12);
    }
}
