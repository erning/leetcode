use std::collections::BinaryHeap;

pub fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
    if grid[0][0] != 0 {
        return -1;
    }
    let n = grid.len();
    let m = n as i32;
    let mut grid = grid;
    let mut queue: BinaryHeap<(i32, i32, i32)> = BinaryHeap::new();
    queue.push((-1, 0, 0));
    grid[0][0] = 2;

    while let Some((c, x, y)) = queue.pop() {
        if x == m - 1 && y == m - 1 {
            return -c;
        }
        for (dx, dy) in [
            (0, -1),
            (1, -1),
            (1, 0),
            (1, 1),
            (0, 1),
            (-1, 1),
            (-1, 0),
            (-1, -1),
        ] {
            let nx = x + dx;
            if nx < 0 || nx >= m {
                continue;
            }
            let ny = y + dy;
            if ny < 0 || ny >= m {
                continue;
            }
            if grid[ny as usize][nx as usize] != 0 {
                continue;
            }
            queue.push((c - 1, nx, ny));
            grid[ny as usize][nx as usize] = 2;
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(input: &[&[i32]], expected: i32) {
        let grid: Vec<Vec<i32>> = input.iter().map(|v| v.to_vec()).collect();
        let output = shortest_path_binary_matrix(grid);
        assert_eq!(output, expected, "{:?}", input);
    }

    #[test]
    fn example() {
        tf(&[&[0, 1], &[1, 0]], 2);
        tf(&[&[0, 0, 0], &[1, 1, 0], &[1, 1, 0]], 4);
        tf(&[&[1, 0, 0], &[1, 1, 0], &[1, 1, 0]], -1);
    }
}
