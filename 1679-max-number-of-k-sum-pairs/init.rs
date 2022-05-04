use std::collections::HashMap;

pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();
    for &v in nums.iter().filter(|&&v| v < k) {
        if let Some(m) = map.get_mut(&v) {
            *m += 1;
        } else {
            map.insert(v, 1);
        }
    }
    let mut count = 0;
    for a in nums.into_iter().filter(|&v| v < k) {
        if let Some(c) = map.get_mut(&a) {
            *c -= 1;
            if *c <= 0 {
                map.remove(&a);
            }
        } else {
            continue;
        }
        let b = k - a;
        if let Some(c) = map.get_mut(&b) {
            *c -= 1;
            if *c <= 0 {
                map.remove(&b);
            }
        } else {
            continue;
        }
        count += 1;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(max_operations(vec![1, 2, 3, 4], 5), 2);
        assert_eq!(max_operations(vec![3, 1, 3, 4, 3], 6), 1);
    }
}
