use std::collections::HashSet;

pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
    fn recursion(i: usize, v: &mut Vec<usize>, nums: &[i32], rv: &mut HashSet<Vec<i32>>) {
        if i >= v.len() {
            rv.insert(v.iter().map(|x| nums[*x]).collect());
            return;
        }
        for j in 0..v.len() {
            if v[..i].contains(&j) {
                continue;
            }
            v[i] = j;
            recursion(i + 1, v, nums, rv);
        }
    }

    let mut rv: HashSet<Vec<i32>> = HashSet::new();
    recursion(0, &mut vec![0; nums.len()], &nums, &mut rv);
    rv.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(permute_unique(vec![1, 1, 2]).len(), 3);
        // permute_unique(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}
