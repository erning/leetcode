pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
    let m = obstacle_grid.len();
    let n = obstacle_grid[0].len();
    if obstacle_grid[0][0] > 0 || obstacle_grid[m - 1][n - 1] > 0 {
        return 0;
    }
    let mut grid = obstacle_grid;
    grid[0][0] = -1;
    for y in 1..m {
        if grid[y][0] == 1 {
            continue;
        }
        if grid[y - 1][0] <= 0 {
            grid[y][0] = grid[y - 1][0];
        }
    }
    for x in 1..n {
        if grid[0][x] == 1 {
            continue;
        }
        if grid[0][x - 1] <= 0 {
            grid[0][x] = grid[0][x - 1];
        }
    }
    for y in 1..m {
        for x in 1..n {
            if grid[y][x] == 1 {
                continue;
            }
            if grid[y][x - 1] <= 0 {
                grid[y][x] += grid[y][x - 1];
            }
            if grid[y - 1][x] <= 0 {
                grid[y][x] += grid[y - 1][x];
            }
        }
    }
    -grid[m - 1][n - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            unique_paths_with_obstacles(vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]]),
            2
        );
        assert_eq!(unique_paths_with_obstacles(vec![vec![0, 1], vec![0, 0]]), 1);

        assert_eq!(unique_paths_with_obstacles(vec![vec![0, 0], vec![0, 1]]), 0);
    }
}
