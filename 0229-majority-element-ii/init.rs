use std::collections::HashMap;

pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
    let p = nums.len() as i32 / 3;
    let mut map: HashMap<i32, i32> = HashMap::new();
    for v in nums.into_iter() {
        if let Some(c) = map.get_mut(&v) {
            *c += 1;
        } else {
            map.insert(v, 1);
        }
    }
    map.into_iter().filter(|v| v.1 > p).map(|v| v.0).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(nums: &[i32], expected: &[i32]) {
        let input = nums.to_vec();
        let mut output = majority_element(input);
        output.sort_unstable();
        assert_eq!(output, expected, "{:?}", nums);
    }

    #[test]
    fn example() {
        tf(&[3, 2, 3], &[3]);
        tf(&[1], &[1]);
        tf(&[1, 2], &[1, 2]);
    }
}
