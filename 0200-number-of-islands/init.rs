pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    fn dfs(x: usize, y: usize, grid: &mut Vec<Vec<char>>) {
        const DIRS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
        let m = grid.len() as i32;
        let n = grid[0].len() as i32;
        if grid[y][x] != '1' {
            return;
        }
        grid[y][x] = '0';
        for d in DIRS.iter() {
            let (nx, ny) = (x as i32 + d.0, y as i32 + d.1);
            if nx < 0 || nx >= n || ny < 0 || ny >= m {
                continue;
            }
            dfs(nx as usize, ny as usize, grid);
        }
    }

    let m = grid.len();
    let n = grid[0].len();
    let mut grid = grid;
    let mut count = 0;
    for y in 0..m {
        for x in 0..n {
            if grid[y][x] == '1' {
                count += 1;
                dfs(x, y, &mut grid);
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            num_islands(vec![
                vec!['1', '1', '1', '1', '0'],
                vec!['1', '1', '0', '1', '0'],
                vec!['1', '1', '0', '0', '0'],
                vec!['0', '0', '0', '0', '0'],
            ]),
            1
        );
        assert_eq!(
            num_islands(vec![
                vec!['1', '1', '0', '0', '0'],
                vec!['1', '1', '0', '0', '0'],
                vec!['0', '0', '1', '0', '0'],
                vec!['0', '0', '0', '1', '1'],
            ]),
            3
        );
    }
}
