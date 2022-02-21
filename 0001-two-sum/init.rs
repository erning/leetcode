use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut m: HashMap<i32, i32> = HashMap::new();
    for (i, &v) in nums.iter().enumerate() {
        let i = i as i32;
        let complement = target - v;
        if let Some(&j) = m.get(&complement) {
            return vec![j, i];
        }
        m.insert(v, i);
    }
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let nums: Vec<i32> = vec![2, 7, 11, 15];
        let target: i32 = 9;
        let expected: Vec<i32> = vec![0, 1];
        let output = two_sum(nums, target);

        assert_eq!(expected, output);
    }
}
