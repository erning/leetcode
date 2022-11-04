pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
    fn dfs(x: i32, y: i32, matrix: &Vec<Vec<i32>>, dp: &mut Vec<Vec<i32>>, max: &mut i32) {
        if dp[y as usize][x as usize] != 0 {
            return;
        }
        let n = matrix[0].len() as i32;
        let m = matrix.len() as i32;

        dp[y as usize][x as usize] = 1;
        let curr = matrix[y as usize][x as usize];
        for (dx, dy) in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
            let nx = x + dx;
            if nx < 0 || nx >= n {
                continue;
            }
            let ny = y + dy;
            if ny < 0 || ny >= m {
                continue;
            }
            let next = matrix[ny as usize][nx as usize];
            if next <= curr {
                continue;
            }
            dfs(nx, ny, matrix, dp, max);
            dp[y as usize][x as usize] =
                i32::max(dp[y as usize][x as usize], dp[ny as usize][nx as usize] + 1);
        }

        if *max < dp[y as usize][x as usize] {
            *max = dp[y as usize][x as usize];
        }
    }

    let n = matrix[0].len() as i32;
    let m = matrix.len() as i32;
    let mut dp = vec![vec![0; n as usize]; m as usize];
    let mut max = 0;

    for y in 0..m {
        for x in 0..n {
            dfs(x, y, &matrix, &mut dp, &mut max);
        }
    }

    max
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(input: &[&[i32]], expected: i32) {
        let matrix: Vec<Vec<_>> = input.iter().map(|v| v.to_vec()).collect();
        let output = longest_increasing_path(matrix);
        assert_eq!(output, expected, "{:?}", input);
    }

    #[test]
    fn example() {
        tf(&[&[9, 9, 4], &[6, 6, 8], &[2, 1, 1]], 4);
        tf(&[&[3, 4, 5], &[3, 2, 6], &[2, 2, 1]], 4);
        tf(&[&[1]], 1);

        tf(&[&[1, 2]], 2);
    }
}
