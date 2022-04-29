pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
    let m = dungeon.len();
    let n = dungeon[0].len();

    let mut prev: Vec<i32> = vec![0; n];
    let mut curr: Vec<i32> = vec![0; n];
    curr[n - 1] = i32::max(1, 1 - dungeon[m - 1][n - 1]);

    for x in (0..n - 1).rev() {
        let v = dungeon[m - 1][x];
        curr[x] = i32::max(1, curr[x + 1] - v);
    }

    std::mem::swap(&mut prev, &mut curr);
    for y in (0..m - 1).rev() {
        let v = dungeon[y][n - 1];
        curr[n - 1] = i32::max(1, prev[n - 1] - v);
        for x in (0..n - 1).rev() {
            let v = dungeon[y][x];
            let a = i32::max(1, curr[x + 1] - v);
            let b = i32::max(1, prev[x] - v);
            curr[x] = i32::min(a, b);
        }
        std::mem::swap(&mut prev, &mut curr);
    }
    prev[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            calculate_minimum_hp(vec![vec![-2, -3, 3], vec![-5, -10, 1], vec![10, 30, -5]]),
            7
        );
        assert_eq!(calculate_minimum_hp(vec![vec![0]]), 1);

        assert_eq!(
            calculate_minimum_hp(vec![vec![1, -3, 3], vec![0, -2, 0], vec![-3, -3, -3]]),
            3
        );
    }
}
