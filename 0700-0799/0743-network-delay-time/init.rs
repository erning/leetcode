use std::collections::BinaryHeap;

pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
    let mut graph: Vec<Vec<(i32, i32)>> = vec![vec![]; n as usize];
    for time in times.into_iter() {
        let (u, v, w) = (time[0], time[1], time[2]);
        graph[(u - 1) as usize].push((v - 1, w));
    }
    let mut visited: Vec<i32> = vec![1; n as usize];
    let mut queue: BinaryHeap<(i32, i32)> = BinaryHeap::new();
    queue.push((0, k - 1));

    while let Some((time, node)) = queue.pop() {
        if visited[node as usize] <= 0 {
            continue;
        }
        visited[node as usize] = time;
        for &(next_node, node_time) in graph[node as usize].iter() {
            queue.push((time - node_time, next_node));
        }
    }
    let mut min = 1;
    for v in visited.into_iter() {
        if v == 1 {
            return -1;
        }
        if v < min {
            min = v;
        }
    }
    -min
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            network_delay_time(vec![vec![2, 1, 1], vec![2, 3, 1], vec![3, 4, 1]], 4, 2),
            2
        );
        assert_eq!(network_delay_time(vec![vec![1, 2, 1]], 2, 1), 1);
        assert_eq!(network_delay_time(vec![vec![1, 2, 1]], 2, 2), -1);

        assert_eq!(
            network_delay_time(vec![vec![1, 2, 1], vec![2, 1, 3]], 2, 2),
            3
        );

        assert_eq!(
            network_delay_time(vec![vec![1, 2, 1], vec![2, 3, 2], vec![1, 3, 2]], 3, 1),
            2
        );
    }
}
