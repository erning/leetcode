pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    let mut intervals = intervals;
    intervals.reverse();
    let mut rv = Vec::new();
    let mut next = new_interval;
    while let Some(mut curr) = intervals.pop() {
        if curr[0] > next[0] {
            std::mem::swap(&mut curr, &mut next);
        }
        if curr[1] < next[0] {
            rv.push(curr);
            continue;
        }
        if curr[0] > next[1] {
            rv.push(next);
            next = curr;
            continue;
        }
        next[0] = curr[0];
        if curr[1] > next[1] {
            next[1] = curr[1];
        }
    }
    rv.push(next);
    rv
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            insert(vec![vec![1, 3], vec![6, 9]], vec![2, 5]),
            vec![vec![1, 5], vec![6, 9]]
        );
        assert_eq!(
            insert(
                vec![
                    vec![1, 2],
                    vec![3, 5],
                    vec![6, 7],
                    vec![8, 10],
                    vec![12, 16]
                ],
                vec![4, 8]
            ),
            vec![vec![1, 2], vec![3, 10], vec![12, 16]]
        );
    }
}
