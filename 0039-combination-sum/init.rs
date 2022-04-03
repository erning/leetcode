use std::collections::HashSet;

pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut dp: Vec<HashSet<Vec<i32>>> = vec![HashSet::new(); target as usize + 1];

    for i in 1..=target {
        for &a in candidates.iter().filter(|&v| i >= *v) {
            if i == a {
                dp[i as usize].insert(vec![i]);
                // dp[i] == [i]
                continue;
            }
            let b = i - a;

            // dp[i] == dp[a] + dp[b];
            let seta = &dp[a as usize];
            let setb = &dp[b as usize];
            if !seta.is_empty() && !setb.is_empty() {
                let mut vv: Vec<Vec<i32>> = Vec::new();
                for va in seta.iter() {
                    for vb in setb.iter() {
                        let mut v = va.clone();
                        v.extend_from_slice(vb);
                        v.sort();
                        vv.push(v);
                    }
                }
                let s = &mut dp[i as usize];
                for v in vv.into_iter() {
                    s.insert(v);
                }
            }
        }
    }

    dp[target as usize].clone().into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(candidates: Vec<i32>, target: i32, expected: Vec<Vec<i32>>) {
        let mut output = combination_sum(candidates, target);
        output.iter_mut().for_each(|v| v.sort());
        output.sort();
        assert_eq!(output, expected);
    }

    #[test]
    fn example() {
        tf(vec![2, 3, 6, 7], 7, vec![vec![2, 2, 3], vec![7]]);
        tf(
            vec![2, 3, 5],
            8,
            vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]],
        );
        tf(vec![2], 1, vec![]);

        tf(vec![1], 1, vec![vec![1]]);
    }
}
