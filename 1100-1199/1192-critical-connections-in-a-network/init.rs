// bridges in a graph
pub fn critical_connections(n: i32, connections: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    fn dfs(
        node: usize,
        parent: usize,
        graph: &Vec<Vec<usize>>,
        time: &mut i32,
        discovery: &mut Vec<i32>,
        low: &mut Vec<i32>,
        answer: &mut Vec<Vec<i32>>,
    ) {
        *time += 1;
        discovery[node] = *time;
        low[node] = *time;

        for &next in graph[node].iter() {
            if next == parent {
                // skip if next is parent
                continue;
            }
            if discovery[next] == 0 {
                // if the next node is not yet visited, discovery it using dfs
                dfs(next, node, graph, time, discovery, low, answer);
            }

            // if next node can be discovered early than the current node,
            // than update the minimum discovery time of current node as well
            low[node] = i32::min(low[node], low[next]);
        }

        // if current node can only be disovered from the parent node, than the
        // parent to current node's edge is critical edge (bridge) */
        if parent != usize::MAX && low[node] > discovery[parent] {
            answer.push(vec![parent as i32, node as i32]);
        }
    }

    let n = n as usize;
    let mut graph: Vec<Vec<usize>> = vec![Vec::new(); n];
    for edge in connections {
        let (a, b) = (edge[0] as usize, edge[1] as usize);
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut time = 1;
    let mut discovery = vec![0; n];
    let mut low = vec![0; n];
    let mut answer = Vec::new();

    dfs(
        0,
        usize::MAX,
        &graph,
        &mut time,
        &mut discovery,
        &mut low,
        &mut answer,
    );

    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(n: i32, input: &[&[i32]], expected: &[&[i32]]) {
        let connections: Vec<Vec<i32>> = input.iter().map(|v| v.to_vec()).collect();
        let mut output = critical_connections(n, connections);
        output.iter_mut().for_each(|v| v.sort_unstable());
        output.sort_unstable();
        assert_eq!(output, expected, "{:?}", (n, input));
    }

    #[test]
    fn example() {
        tf(4, &[&[0, 1], &[1, 2], &[2, 0], &[1, 3]], &[&[1, 3]]);
        tf(22, &[&[0, 1]], &[&[0, 1]]);
    }
}
