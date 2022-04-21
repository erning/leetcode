use std::collections::HashMap;

pub struct LRUCache {
    cap: usize,
    map: HashMap<i32, (usize, i32)>,
    count: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        LRUCache {
            cap: capacity as usize,
            map: HashMap::with_capacity(capacity as usize),
            count: 0,
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        if let Some((c, v)) = self.map.get_mut(&key) {
            self.count += 1;
            *c = self.count;
            *v
        } else {
            -1
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        self.count += 1;
        self.map.insert(key, (self.count, value));
        if self.map.len() > self.cap {
            if let Some(k) = self.map.iter().min_by(|a, b| a.1 .0.cmp(&b.1 .0)) {
                let k = *k.0;
                self.map.remove(&k);
            }
        }
    }
}

/*
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 1);
        cache.put(2, 2);
        assert_eq!(cache.get(1), 1);
        cache.put(3, 3);
        assert_eq!(cache.get(2), -1);
        cache.put(4, 4);
        assert_eq!(cache.get(1), -1);
        assert_eq!(cache.get(3), 3);
        assert_eq!(cache.get(4), 4);
    }

    #[test]
    fn example2() {
        // ["LRUCache","get","put","get","put","put","get","get"]
        // [[2],       [2],  [2,6],[1],  [1,5],[1,2],[1],  [2]]
        let mut cache = LRUCache::new(2);
        assert_eq!(cache.get(2), -1);
        cache.put(2, 6);
        assert_eq!(cache.get(1), -1);
        cache.put(1, 5);
        cache.put(1, 2);
        assert_eq!(cache.get(1), 2);
        assert_eq!(cache.get(2), 6);
    }
}
