use std::collections::HashSet;

pub fn maximum_score(scores: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
    let n = scores.len();
    let mut graph: Vec<HashSet<usize>> = vec![HashSet::new(); n];
    for v in edges.iter() {
        graph[v[0] as usize].insert(v[1] as usize);
        graph[v[1] as usize].insert(v[0] as usize);
    }

    let graph: Vec<Vec<usize>> = graph
        .into_iter()
        .map(|v| {
            let mut v: Vec<usize> = v.into_iter().collect();
            v.sort_unstable_by_key(|&i| -scores[i]);
            v
        })
        .collect();

    let mut max = -1;
    for v in edges.iter() {
        let a = v[0] as usize;
        let b = v[1] as usize;
        for &c in graph[a].iter() {
            if c == b {
                continue;
            }
            for &d in graph[b].iter() {
                if d == a || d == c {
                    continue;
                }
                let sum = scores[a] + scores[b] + scores[c] + scores[d];
                if sum > max {
                    max = sum
                }
                break;
            }
        }
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            maximum_score(
                vec![5, 2, 9, 8, 4],
                vec![
                    vec![0, 1],
                    vec![1, 2],
                    vec![2, 3],
                    vec![0, 2],
                    vec![1, 3],
                    vec![2, 4]
                ]
            ),
            24
        );
        assert_eq!(
            maximum_score(
                vec![9, 20, 6, 4, 11, 12],
                vec![vec![0, 3], vec![5, 3], vec![2, 4], vec![1, 3]]
            ),
            -1
        );

        assert_eq!(
            maximum_score(
                vec![8, 4, 3, 8, 5],
                vec![
                    vec![3, 0],
                    vec![2, 1],
                    vec![2, 0],
                    vec![4, 2],
                    vec![3, 4],
                    vec![4, 0],
                    vec![2, 3],
                    vec![1, 3]
                ]
            ),
            25
        );
    }
}
