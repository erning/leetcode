pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
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
            let mut new_comb = comb.clone();
            new_comb.push(a);
            recursion(rv, &candidates[i..], target - a, new_comb);
        }
        //
    }
    recursion(&mut rv, candidates.as_slice(), target, vec![]);
    rv
}

// pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
//     let mut dp: Vec<Option<Vec<Vec<i32>>>> = vec![None; target as usize + 1];
//
//     let mut vvi: HashSet<Vec<i32>> = HashSet::new();
//     for i in 1..=target {
//         for &a in candidates.iter() {
//             if i < a {
//                 continue;
//             }
//             if i == a {
//                 // dp[i] += [i]
//                 vvi.insert(vec![i]);
//                 continue;
//             }
//             let b = i - a;
//             if b < a {
//                 continue;
//             }
//             if dp[b as usize].is_none() {
//                 continue;
//             }
//
//             // dp[i] += dp[a] + dp[b];
//             for va in dp[a as usize].as_ref().unwrap().iter() {
//                 for vb in dp[b as usize].as_ref().unwrap().iter() {
//                     if va[0] > vb[0] {
//                         continue;
//                     }
//                     let mut vi = Vec::new();
//                     vi.extend(va.into_iter());
//                     vi.extend(vb.into_iter());
//                     vi.sort();
//                     vvi.insert(vi);
//                 }
//             }
//         }
//         if !vvi.is_empty() {
//             dp[i as usize].replace(vvi.drain().collect());
//         }
//     }
//
//     dp.pop().unwrap().unwrap_or(vec![])
// }

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(candidates: Vec<i32>, target: i32, expected: Vec<Vec<i32>>) {
        let sum = combination_sum(candidates, target);
        let mut output = sum.clone();
        output.iter_mut().for_each(|v| v.sort());
        output.sort();
        assert_eq!(output, expected, "sum: {:?}", &sum);
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

        tf(
            vec![1, 2],
            4,
            vec![vec![1, 1, 1, 1], vec![1, 1, 2], vec![2, 2]],
        );
        tf(
            vec![2, 7, 6, 3, 5, 1],
            9,
            vec![
                vec![1, 1, 1, 1, 1, 1, 1, 1, 1],
                vec![1, 1, 1, 1, 1, 1, 1, 2],
                vec![1, 1, 1, 1, 1, 1, 3],
                vec![1, 1, 1, 1, 1, 2, 2],
                vec![1, 1, 1, 1, 2, 3],
                vec![1, 1, 1, 1, 5],
                vec![1, 1, 1, 2, 2, 2],
                vec![1, 1, 1, 3, 3],
                vec![1, 1, 1, 6],
                vec![1, 1, 2, 2, 3],
                vec![1, 1, 2, 5],
                vec![1, 1, 7],
                vec![1, 2, 2, 2, 2],
                vec![1, 2, 3, 3],
                vec![1, 2, 6],
                vec![1, 3, 5],
                vec![2, 2, 2, 3],
                vec![2, 2, 5],
                vec![2, 7],
                vec![3, 3, 3],
                vec![3, 6],
            ],
        );

        tf(vec![1], 1, vec![vec![1]]);
    }
}
