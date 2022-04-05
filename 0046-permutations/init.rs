pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    fn recursion(i: usize, v: &mut Vec<usize>, nums: &[i32], rv: &mut Vec<Vec<i32>>) {
        if i >= v.len() {
            rv.push(v.iter().map(|x| nums[*x]).collect());
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

    let mut rv: Vec<Vec<i32>> = Vec::new();
    recursion(0, &mut vec![0; nums.len()], &nums, &mut rv);
    rv
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        permute(vec![1, 2, 3, 4]);
        permute(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}
