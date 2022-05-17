pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut used: Vec<bool> = vec![false; nums.len()];
    let mut rv: Vec<Vec<i32>> = Vec::new();
    fn recursion(
        i: usize,
        v: Vec<usize>,
        nums: &[i32],
        rv: &mut Vec<Vec<i32>>,
        used: &mut Vec<bool>,
    ) {
        rv.push(v.iter().map(|&v| nums[v]).collect());
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
    recursion(0, vec![], &nums, &mut rv, &mut used);
    rv
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(nums: Vec<i32>, expected: Vec<Vec<i32>>) {
        let mut output = subsets(nums.clone());
        output.sort();
        assert_eq!(output, expected, "{:?}", nums);
    }

    #[test]
    fn example() {
        tf(
            vec![1, 2, 3],
            vec![
                vec![],
                vec![1],
                vec![1, 2],
                vec![1, 2, 3],
                vec![1, 3],
                vec![2],
                vec![2, 3],
                vec![3],
            ],
        );
    }
}
