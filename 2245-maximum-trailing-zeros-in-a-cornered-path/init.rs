pub fn max_trailing_zeros(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();

    let factors: Vec<Vec<(i32, i32)>> = grid
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(|mut v| {
                    let mut a = 0;
                    let mut b = 0;
                    while v % 2 == 0 {
                        a += 1;
                        v /= 2;
                    }
                    while v % 5 == 0 {
                        b += 1;
                        v /= 5;
                    }
                    (a, b)
                })
                .collect()
        })
        .collect();

    macro_rules! hs {
        ($prev:expr, $curr:expr) => {
            let prev = $prev;
            let curr = &mut $curr;
            curr.0 += prev.0;
            curr.1 += prev.1;
        };
    }

    let mut uhs: Vec<Vec<(i32, i32)>> = factors.clone();
    let mut dhs: Vec<Vec<(i32, i32)>> = factors.clone();
    for y in 1..m {
        for x in 0..n {
            hs!(uhs[y - 1][x], uhs[y][x]);
            hs!(dhs[m - y][x], dhs[m - y - 1][x]);
        }
    }

    let mut lhs: Vec<Vec<(i32, i32)>> = factors.clone();
    let mut rhs: Vec<Vec<(i32, i32)>> = factors.clone();
    for x in 1..n {
        for y in 0..m {
            hs!(lhs[y][x - 1], lhs[y][x]);
            hs!(rhs[y][n - x], rhs[y][n - x - 1]);
        }
    }

    let mut max = 0;
    for y in 0..m {
        for x in 0..n {
            let mut max_zero = |a: &Vec<Vec<(i32, i32)>>, b: &Vec<Vec<(i32, i32)>>| {
                let a = a[y][x];
                let b = b[y][x];
                let c = factors[y][x];
                max = max.max(i32::min(a.0 + b.0 - c.0, a.1 + b.1 - c.1));
            };
            max_zero(&uhs, &rhs);
            max_zero(&rhs, &dhs);
            max_zero(&dhs, &lhs);
            max_zero(&lhs, &uhs);
        }
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(input: Vec<Vec<i32>>, excepted: i32) {
        let grid = input.clone();
        let output = max_trailing_zeros(grid);
        assert_eq!(output, excepted, "{:?}", input);
    }

    #[test]
    fn example() {
        tf(
            vec![
                vec![23, 17, 15, 3, 20],
                vec![8, 1, 20, 27, 11],
                vec![9, 4, 6, 2, 21],
                vec![40, 9, 1, 10, 6],
                vec![22, 7, 4, 5, 3],
            ],
            3,
        );

        tf(vec![vec![4, 3, 2], vec![7, 6, 1], vec![8, 8, 8]], 0)
    }
}
