use std::collections::HashSet;

pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut dp: Vec<Vec<Vec<i32>>> = vec![Vec::new(); target as usize + 1];

    for i in 1..=target {
        let mut vvi: Vec<Vec<i32>> = Vec::new();
        for &a in candidates.iter() {
            if i < a {
                continue;
            }
            if i == a {
                // dp[i] += [i]
                vvi.push(vec![i]);
                continue;
            }
            let b = i - a;
            if b < a {
                continue;
            }

            // dp[i] += dp[a] + dp[b];
            for vb in dp[b as usize].iter() {
                for va in dp[a as usize].iter() {
                    let mut vi = Vec::new();
                    vi.extend(va.iter());
                    vi.extend(vb.iter());
                    vi.sort();
                    vvi.push(vi);
                }
            }
        }
        let set: HashSet<_> = vvi.drain(..).collect();
        dp[i as usize].extend(set.into_iter());
    }

    dp.pop().unwrap()
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
