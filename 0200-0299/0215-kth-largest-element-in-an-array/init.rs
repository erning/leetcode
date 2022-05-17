use std::cmp::Ordering;

pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    let k = k as usize - 1;
    let n = nums.len();
    let mut nums = nums;
    let mut lo = 0;
    let mut hi = n - 1;
    while lo < hi {
        let m = lo + (hi - lo) / 2;
        nums.swap(m, hi);
        let pivot = nums[hi];
        let mut p = lo;
        for i in lo..hi {
            if nums[i] > pivot {
                nums.swap(p, i);
                p += 1;
            }
        }
        nums.swap(p, hi);
        match p.cmp(&k) {
            Ordering::Less => lo = p + 1,
            Ordering::Greater => hi = p - 1,
            Ordering::Equal => break,
        }
    }
    nums[k]
}

use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn find_kth_largest_heap(nums: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;
    let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::with_capacity(k + 1);
    for n in nums.into_iter() {
        heap.push(Reverse(n));
        if heap.len() > k {
            heap.pop();
        }
    }
    if let Some(Reverse(n)) = heap.pop() {
        n
    } else {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2), 5);
        assert_eq!(find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4), 4);

        assert_eq!(find_kth_largest(vec![7, 6, 5, 4, 3, 2, 1], 2), 6);
    }
}
