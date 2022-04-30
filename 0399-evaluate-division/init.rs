use std::collections::HashMap;

pub fn calc_equation(
    equations: Vec<Vec<String>>,
    values: Vec<f64>,
    queries: Vec<Vec<String>>,
) -> Vec<f64> {
    let mut map: HashMap<&str, usize> = HashMap::new();
    for e in equations.iter() {
        let a = e[0].as_str();
        let b = e[1].as_str();
        if !map.contains_key(a) {
            map.insert(a, map.len());
        }
        if !map.contains_key(b) {
            map.insert(b, map.len());
        }
    }
    let n = map.len();
    let mut table: Vec<Vec<Option<f64>>> = vec![vec![None; n]; n];
    for (i, e) in equations.iter().enumerate() {
        let a = e[0].as_str();
        let b = e[1].as_str();
        let v = values[i];
        let &y = map.get(a).unwrap();
        let &x = map.get(b).unwrap();
        table[y][x].replace(v);
        table[x][y].replace(1.0 / v);
    }

    loop {
        let mut done = true;
        for a in 0..n {
            table[a][a].replace(1.0);
            for b in 0..n {
                if let Some(ab) = table[a][b] {
                    for c in 0..n {
                        if table[a][c].is_some() {
                            continue;
                        }
                        if let Some(bc) = table[b][c] {
                            table[a][c].replace(ab * bc);
                            table[c][a].replace(1.0 / (ab * bc));
                            done = false;
                        }
                    }
                }
            }
        }
        if done {
            break;
        }
    }

    queries
        .into_iter()
        .map(|v| {
            let a = v[0].as_str();
            let b = v[1].as_str();
            if let (Some(&a), Some(&b)) = (map.get(a), map.get(b)) {
                if let Some(value) = table[a][b] {
                    value
                } else {
                    -1.0
                }
            } else {
                -1.0
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(equations: &[[&str; 2]], values: &[f64], queries: &[[&str; 2]], excepted: &[f64]) {
        let output = calc_equation(
            equations
                .into_iter()
                .map(|v| v.into_iter().map(|s| s.to_string()).collect())
                .collect(),
            values.to_vec(),
            queries
                .into_iter()
                .map(|v| v.into_iter().map(|s| s.to_string()).collect())
                .collect(),
        );
        assert_eq!(output, excepted, "{:?}", equations);
    }

    #[test]
    fn example() {
        tf(
            &[["a", "b"], ["b", "c"], ["bc", "cd"]],
            &[1.5, 2.5, 5.0],
            &[["a", "c"], ["c", "b"], ["bc", "cd"], ["cd", "bc"]],
            &[3.75000, 0.40000, 5.00000, 0.20000],
        );

        tf(
            &[["a", "b"], ["b", "c"]],
            &[2.0, 3.0],
            &[["a", "c"], ["b", "a"], ["a", "e"], ["a", "a"], ["x", "x"]],
            &[6.00000, 0.50000, -1.00000, 1.00000, -1.00000],
        );
    }
}
