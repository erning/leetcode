pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut intervals = intervals;
    intervals.sort_by(|a, b| b[0].cmp(&a[0]));
    let mut rv = Vec::new();
    let mut prev = intervals.pop().unwrap();
    while let Some(curr) = intervals.pop() {
        if prev[1] < curr[0] {
            rv.push(prev);
            prev = curr;
        } else if prev[1] < curr[1] {
            prev[1] = curr[1];
        }
    }
    rv.push(prev);
    rv
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]]),
            vec![vec![1, 6], vec![8, 10], vec![15, 18]]
        );
        assert_eq!(merge(vec![vec![1, 4], vec![4, 5]]), vec![vec![1, 5]]);

        assert_eq!(merge(vec![vec![1, 4], vec![0, 4]]), vec![vec![0, 4]]);
        assert_eq!(
            merge(vec![
                vec![2, 3],
                vec![4, 5],
                vec![6, 7],
                vec![8, 9],
                vec![1, 10]
            ]),
            vec![vec![1, 10]]
        );
    }
}
