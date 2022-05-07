use std::collections::HashMap;

pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, k: i32, t: i32) -> bool {
    struct BucketSet {
        t: i64,
        map: HashMap<i64, i64>,
    }

    impl BucketSet {
        fn new(t: i32) -> Self {
            let t = t as i64;
            let map = HashMap::new();
            BucketSet { t, map }
        }

        #[inline]
        fn slot(&self, n: i32) -> i64 {
            (n as i64 + u32::MAX as i64) / (self.t + 1)
        }

        fn insert(&mut self, n: i32) {
            let i = self.slot(n);
            self.map.insert(i, n as i64);
        }

        fn remove(&mut self, n: i32) {
            let i = self.slot(n);
            self.map.remove(&i);
        }

        fn contains(&self, n: i32) -> bool {
            let i = self.slot(n);
            if self.map.contains_key(&i) {
                return true;
            }
            if let Some(v) = self.map.get(&(i - 1)) {
                if i64::abs(v - n as i64) <= self.t {
                    return true;
                }
            }
            if let Some(v) = self.map.get(&(i + 1)) {
                if i64::abs(v - n as i64) <= self.t {
                    return true;
                }
            }
            false
        }
    }

    let mut set: BucketSet = BucketSet::new(t);
    let k = k as usize;
    for &n in nums.iter().take(k + 1) {
        if set.contains(n) {
            return true;
        }
        set.insert(n);
    }
    let mut i = 0;
    for &n in nums.iter().skip(k + 1) {
        set.remove(nums[i]);
        if set.contains(n) {
            return true;
        }
        set.insert(n);
        i += 1;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(nums: &[i32], k: i32, t: i32, expected: bool) {
        let output = contains_nearby_almost_duplicate(nums.to_vec(), k, t);
        assert_eq!(output, expected, "{:?}", nums);
    }

    #[test]
    fn example() {
        tf(&[1, 2, 3, 1], 3, 0, true);
        tf(&[1, 0, 1, 1], 1, 2, true);
        tf(&[1, 5, 9, 1, 5, 9], 2, 3, false);

        tf(&[8, 7, 15, 1, 6, 1, 9, 15], 1, 3, true);
        tf(&[2147483647, -1, 2147483647], 1, 2147483647, false);

        tf(&[-2, 1, 3, 5, 7], 3, 1, false);
    }
}
