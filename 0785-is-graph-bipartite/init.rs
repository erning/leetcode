pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
    fn travel(i: usize, color: i32, graph: &Vec<Vec<i32>>, visited: &mut Vec<i32>) -> bool {
        let c = visited[i];
        if c != 0 {
            return c == color;
        }
        visited[i] = color;
        for &j in graph[i].iter() {
            if !travel(j as usize, -color, graph, visited) {
                return false;
            }
        }
        true
    }

    let mut visited: Vec<i32> = vec![0; graph.len()];
    for i in 0..graph.len() {
        if visited[i] == 0 {
            if !travel(i, 1, &graph, &mut visited) {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            is_bipartite(vec![vec![1, 2, 3], vec![0, 2], vec![0, 1, 3], vec![0, 2]]),
            false
        );
        assert_eq!(
            is_bipartite(vec![vec![1, 3], vec![0, 2], vec![1, 3], vec![0, 2]]),
            true
        );
    }
}
