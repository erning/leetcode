use std::collections::BinaryHeap;
use std::collections::HashSet;

pub fn minimum_obstacles(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len() as i32;
    let n = grid[0].len() as i32;

    let mut queue: BinaryHeap<(i32, (i32, i32))> = BinaryHeap::new();
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    queue.push((-grid[0][0], (0, 0)));
    visited.insert((0, 0));

    while let Some((v, (x, y))) = queue.pop() {
        if x == n - 1 && y == m - 1 {
            return -v;
        }
        for (dx, dy) in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
            let nx = x + dx;
            if nx < 0 || nx >= n {
                continue;
            }
            let ny = y + dy;
            if ny < 0 || ny >= m {
                continue;
            }
            if visited.contains(&(nx, ny)) {
                continue;
            }
            visited.insert((nx, ny));
            queue.push((v - grid[ny as usize][nx as usize], (nx, ny)));
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(input: &[&[i32]], expected: i32) {
        let grid: Vec<Vec<i32>> = input.iter().map(|v| v.iter().copied().collect()).collect();
        let output = minimum_obstacles(grid);
        assert_eq!(output, expected);
    }

    #[test]
    fn example() {
        tf(&[&[0, 1, 1], &[1, 1, 0], &[1, 1, 0]], 2);
        tf(&[&[0, 1, 0, 0, 0], &[0, 1, 0, 1, 0], &[0, 0, 0, 1, 0]], 0);
    }
}
