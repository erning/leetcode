pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let m = m as usize;
    let n = n as usize;
    let mut i = 0;
    let mut j = 0;

    let mut nums3 = Vec::new();
    for v1 in nums1.iter_mut().take(m) {
        if i >= n && j >= nums3.len() {
            break;
        }
        let v2 = if i < n { nums2[i] } else { i32::MAX };
        let v3 = if j < nums3.len() { nums3[j] } else { i32::MAX };
        if v2 < v3 && v2 < *v1 {
            nums3.push(*v1);
            *v1 = v2;
            i += 1;
        } else if v3 < *v1 {
            nums3.push(*v1);
            *v1 = v3;
            j += 1;
        }
    }
    for v1 in nums1.iter_mut().skip(m) {
        let v2 = if i < n { nums2[i] } else { i32::MAX };
        let v3 = if j < nums3.len() { nums3[j] } else { i32::MAX };
        if v2 < v3 {
            *v1 = v2;
            i += 1;
        } else {
            *v1 = v3;
            j += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(nums1: Vec<i32>, nums2: Vec<i32>, expected: Vec<i32>) {
        let mut n1 = nums1.clone();
        let mut n2 = nums2.clone();
        let m = nums1.len();
        let n = nums2.len();
        n1.resize_with(m + n, || 0);
        merge(&mut n1, m as i32, &mut n2, n as i32);
        assert_eq!(n1, expected, "{:?}", (nums1, nums2));
    }

    #[test]
    fn example() {
        tf(vec![1, 2, 3], vec![2, 5, 6], vec![1, 2, 2, 3, 5, 6]);
        tf(vec![1], vec![], vec![1]);
        tf(vec![], vec![1], vec![1]);

        tf(vec![1, 2, 3, 4], vec![5, 6, 7], vec![1, 2, 3, 4, 5, 6, 7]);
        tf(vec![1, 2, 3], vec![4, 5, 6, 7], vec![1, 2, 3, 4, 5, 6, 7]);

        tf(vec![4, 5, 6], vec![1, 2, 3], vec![1, 2, 3, 4, 5, 6]);
        tf(vec![1, 3, 5], vec![2, 4, 6], vec![1, 2, 3, 4, 5, 6]);

        tf(vec![1, 3], vec![2, 4, 5, 6], vec![1, 2, 3, 4, 5, 6]);
        tf(vec![2, 4, 5, 6], vec![1, 3], vec![1, 2, 3, 4, 5, 6]);
    }
}
