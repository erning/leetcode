pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
    let mut m = 0;
    for &n in nums.iter() {
        m ^= n;
    }
    let mut a = 0;
    let mut b = 0;
    m &= !(m - 1);
    for n in nums.into_iter() {
        if n & m == 0 {
            a ^= n;
        } else {
            b ^= n;
        }
    }
    vec![a, b]
}

use std::collections::HashSet;
#[allow(dead_code)]
fn by_hashset(nums: Vec<i32>) -> Vec<i32> {
    let mut set: HashSet<i32> = HashSet::new();
    for n in nums.into_iter() {
        if !set.remove(&n) {
            set.insert(n);
        }
    }
    set.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(input: &[i32], expected: &[i32]) {
        let mut output = single_number(input.to_vec());
        output.sort_unstable();
        assert_eq!(output, expected, "{:?}", input);
    }

    #[test]
    fn example() {
        tf(&[1, 2, 1, 3, 2, 5], &[3, 5]);
        tf(&[-1, 0], &[-1, 0]);
        tf(&[0, 1], &[0, 1]);
    }
}
