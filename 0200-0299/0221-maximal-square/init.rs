pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
    let m = matrix.len();
    let n = matrix[0].len();
    let mut max = 0;
    let mut prev = vec![0; n + 1];
    let mut curr = vec![0; n + 1];
    for row in matrix.iter().take(m) {
        for i in 0..n {
            if row[i] == '0' {
                curr[i + 1] = 0;
                continue;
            }
            let a = curr[i];
            let b = prev[i + 1];
            let c = prev[i];
            let v = a.min(b).min(c) + 1;
            curr[i + 1] = v;
            if v > max {
                max = v
            }
        }
        std::mem::swap(&mut prev, &mut curr);
    }
    max * max
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(matrix: &[&[char]], expected: i32) {
        let input: Vec<Vec<char>> = matrix.iter().map(|v| v.to_vec()).collect();
        let output = maximal_square(input);
        assert_eq!(output, expected, "{:?}", matrix);
    }
    #[test]
    fn example() {
        tf(
            &[
                &['1', '0', '1', '0', '0'],
                &['1', '0', '1', '1', '1'],
                &['1', '1', '1', '1', '1'],
                &['1', '0', '0', '1', '0'],
            ],
            4,
        );
        tf(&[&['0', '1'], &['1', '0']], 1);
        tf(&[&['0']], 0);

        tf(
            &[
                &['1', '0', '1', '0'],
                &['1', '0', '1', '1'],
                &['1', '0', '1', '1'],
                &['1', '1', '1', '1'],
            ],
            4,
        );
    }
}
