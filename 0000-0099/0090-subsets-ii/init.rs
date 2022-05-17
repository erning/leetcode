use std::collections::HashSet;

pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
    fn recursion(
        i: usize,
        v: Vec<usize>,
        nums: &[i32],
        rv: &mut HashSet<Vec<i32>>,
        used: &mut Vec<bool>,
    ) {
        rv.insert(v.iter().map(|&v| nums[v]).collect());
        if v.len() >= nums.len() {
            return;
        }
        for j in i..nums.len() {
            if used[j] {
                continue;
            }
            let mut nv = v.clone();
            nv.push(j);
            used[j] = true;
            recursion(j, nv, nums, rv, used);
            used[j] = false;
        }
    }
    let mut nums = nums;
    nums.sort_unstable();
    let mut used: Vec<bool> = vec![false; nums.len()];
    let mut rv: HashSet<Vec<i32>> = HashSet::new();
    recursion(0, vec![], &nums, &mut rv, &mut used);
    rv.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(nums: Vec<i32>, expected: Vec<Vec<i32>>) {
        let mut output = subsets_with_dup(nums.clone());
        output.sort();
        assert_eq!(output, expected, "{:?}", nums);
    }

    #[test]
    fn example() {
        tf(
            vec![1, 2, 2],
            vec![
                vec![],
                vec![1],
                vec![1, 2],
                vec![1, 2, 2],
                vec![2],
                vec![2, 2],
            ],
        );

        tf(vec![0], vec![vec![], vec![0]]);

        tf(
            vec![4, 4, 4, 1, 4],
            vec![
                vec![],
                vec![1],
                vec![1, 4],
                vec![1, 4, 4],
                vec![1, 4, 4, 4],
                vec![1, 4, 4, 4, 4],
                vec![4],
                vec![4, 4],
                vec![4, 4, 4],
                vec![4, 4, 4, 4],
            ],
        );
    }
}
