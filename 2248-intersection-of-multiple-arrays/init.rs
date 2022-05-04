pub fn intersection(nums: Vec<Vec<i32>>) -> Vec<i32> {
    let n = nums.len() as i32;
    let mut map = [0; 1001];
    for num in nums.into_iter() {
        for v in num.into_iter() {
            map[v as usize] += 1;
        }
    }
    map.iter()
        .enumerate()
        .filter(|(_, &v)| v == n)
        .map(|(i, _)| i as i32)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(input: Vec<Vec<i32>>, expected: Vec<i32>) {
        let nums = input.clone();
        let output = intersection(nums);
        assert_eq!(output, expected, "{:?}", input);
    }

    #[test]
    fn example() {
        tf(
            vec![vec![3, 1, 2, 4, 5], vec![1, 2, 3, 4], vec![3, 4, 5, 6]],
            vec![3, 4],
        );
        tf(vec![vec![1, 2, 3], vec![4, 5, 6]], vec![])
    }
}
