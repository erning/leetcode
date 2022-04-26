pub fn max_trailing_zeros(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();

    let mut factors: Vec<Vec<(i32, i32)>> = Vec::with_capacity(m);
    for y in 0..m {
        let mut row: Vec<(i32, i32)> = Vec::with_capacity(n);
        for x in 0..n {
            let mut a = 0;
            let mut b = 0;
            let mut num = grid[y][x];
            while num % 2 == 0 {
                a += 1;
                num /= 2;
            }
            while num % 5 == 0 {
                b += 1;
                num /= 5;
            }
            row.push((a, b));
        }
        factors.push(row);
    }

    let mut uhs: Vec<Vec<(i32, i32)>> = vec![vec![(0, 0); n]; m];
    let mut dhs: Vec<Vec<(i32, i32)>> = vec![vec![(0, 0); n]; m];
    for x in 0..n {
        uhs[0][x] = factors[0][x];
        dhs[m - 1][x] = factors[m - 1][x];
    }
    for y in 1..m {
        for x in 0..n {
            let (a1, b1) = uhs[y - 1][x];
            let (a2, b2) = factors[y][x];
            uhs[y][x] = (a1 + a2, b1 + b2);

            let (a1, b1) = dhs[m - y][x];
            let (a2, b2) = factors[m - y - 1][x];
            dhs[m - y - 1][x] = (a1 + a2, b1 + b2);
        }
    }

    let mut lhs: Vec<Vec<(i32, i32)>> = vec![vec![(0, 0); n]; m];
    let mut rhs: Vec<Vec<(i32, i32)>> = vec![vec![(0, 0); n]; m];
    for y in 0..m {
        lhs[y][0] = factors[y][0];
        rhs[y][n - 1] = factors[y][n - 1];
    }
    for x in 1..n {
        for y in 0..m {
            let (a1, b1) = lhs[y][x - 1];
            let (a2, b2) = factors[y][x];
            lhs[y][x] = (a1 + a2, b1 + b2);

            let (a1, b1) = rhs[y][n - x];
            let (a2, b2) = factors[y][n - x - 1];
            rhs[y][n - x - 1] = (a1 + a2, b1 + b2);
        }
    }

    let mut max = 0;
    for y in 0..m {
        let (a, b) = rhs[y][0];
        let z = a.min(b);
        if z > max {
            max = z;
        }
    }
    for x in 0..n {
        let (a, b) = dhs[0][x];
        let z = a.min(b);
        if z > max {
            max = z;
        }
    }
    for y in 0..m {
        for x in 0..n {
            let mut max_zero = |a: &Vec<Vec<(i32, i32)>>, b: &Vec<Vec<(i32, i32)>>| {
                let a = a[y][x];
                let b = b[y][x];
                let c = factors[y][x];
                let z = i32::min(a.0 + b.0 - c.0, a.1 + b.1 - c.1);
                if z > max {
                    max = z;
                }
            };
            max_zero(&uhs, &rhs); // up -> right
            max_zero(&rhs, &dhs); // right -> down
            max_zero(&dhs, &lhs); // down -> left
            max_zero(&lhs, &uhs); // left -> up
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
