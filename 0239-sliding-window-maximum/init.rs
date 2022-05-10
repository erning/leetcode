use std::collections::BinaryHeap;
use std::collections::HashMap;

pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let mut answer = Vec::with_capacity(k);

    let mut heap: BinaryHeap<i32> = BinaryHeap::new();
    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut max = i32::MIN;
    for &v in nums.iter().take(k) {
        if v > max {
            max = v;
        }
        heap.push(v);
        if let Some(a) = map.get_mut(&v) {
            *a += 1;
        } else {
            map.insert(v, 1);
        }
    }
    answer.push(max);
    for i in k + 1..=nums.len() {
        // remove from map
        let removed = nums[i - k - 1];
        if let Some(a) = map.get_mut(&removed) {
            if *a > 1 {
                *a -= 1;
            } else {
                map.remove(&removed);
            }
        }
        // add to map
        let added = nums[i - 1];
        if let Some(a) = map.get_mut(&added) {
            *a += 1;
        } else {
            map.insert(added, 1);
        }
        heap.push(added);

        if added > max {
            max = added;
        } else if removed == max && added < max {
            loop {
                max = *heap.peek().unwrap();
                if map.contains_key(&max) {
                    break;
                }
                heap.pop();
            }
        }

        answer.push(max);
    }
    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(input: &[i32], k: i32, expected: &[i32]) {
        let output = max_sliding_window(input.to_vec(), k);
        assert_eq!(output, expected, "{:?}", (input, k));
    }

    #[test]
    fn example() {
        tf(&[1, 3, -1, -3, 5, 3, 6, 7], 3, &[3, 3, 5, 5, 6, 7]);
        tf(&[1], 1, &[1]);

        tf(&[1, 3, 1, 2, 0, 5], 3, &[3, 3, 2, 5]);
    }
}
