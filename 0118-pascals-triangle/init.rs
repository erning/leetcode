pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    let mut rows: Vec<Vec<i32>> = vec![];
    let mut row: Vec<i32> = vec![1];

    fn generate_next(prev: &[i32]) -> Vec<i32> {
        let len = prev.len() + 1;
        let mut row = vec![1; len];
        let m = (len - 1) / 2;
        for i in 1..=m {
            let v = prev[i - 1] + prev[i];
            row[i] = v;
            row[len - i - 1] = v;
        }
        row
    }

    for _ in 0..num_rows {
        let next = generate_next(&row);
        rows.push(row);
        row = next;
    }
    rows
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(input: i32, expected: Vec<Vec<i32>>) {
        let output = generate(input);
        assert_eq!(output, expected);
    }

    #[test]
    fn example() {
        tf(1, vec![vec![1]]);
        tf(
            5,
            vec![
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1],
            ],
        );
    }
}
