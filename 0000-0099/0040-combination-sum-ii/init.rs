pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut candidates = candidates;
    candidates.sort_unstable();
    let mut rv: Vec<Vec<i32>> = Vec::new();

    fn recursion(rv: &mut Vec<Vec<i32>>, candidates: &[i32], target: i32, comb: Vec<i32>) {
        if target == 0 {
            rv.push(comb);
            return;
        }
        for (i, &a) in candidates.iter().enumerate() {
            if target < a {
                continue;
            }
            if i > 0 && a == candidates[i - 1] {
                continue;
            }
            let mut new_comb = comb.clone();
            new_comb.push(a);
            recursion(rv, &candidates[i + 1..], target - a, new_comb);
        }
    }

    recursion(&mut rv, candidates.as_slice(), target, vec![]);
    rv.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(candidates: Vec<i32>, target: i32, expected: Vec<Vec<i32>>) {
        let mut output = combination_sum2(candidates, target);
        output.iter_mut().for_each(|v| v.sort());
        output.sort();
        assert_eq!(output, expected);
    }

    #[test]
    fn example() {
        tf(
            vec![10, 1, 2, 7, 6, 1, 5],
            8,
            vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]],
        );
        tf(vec![2, 5, 2, 1, 2], 5, vec![vec![1, 2, 2], vec![5]]);
        //
        tf(
            vec![4, 4, 2, 1, 4, 2, 2, 1, 3],
            6,
            vec![
                vec![1, 1, 2, 2],
                vec![1, 1, 4],
                vec![1, 2, 3],
                vec![2, 2, 2],
                vec![2, 4],
            ],
        );
        tf(
            vec![
                1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            ],
            27,
            vec![],
        );
    }
}
