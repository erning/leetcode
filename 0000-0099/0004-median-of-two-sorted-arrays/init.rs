use std::cmp::max;
use std::cmp::min;

pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    // ensure length of nums1 <= length of nums2
    let (nums1, nums2) = if nums1.len() > nums2.len() {
        (nums2, nums1)
    } else {
        (nums1, nums2)
    };
    let len1 = nums1.len();
    let len2 = nums2.len();

    if len2 == 0 {
        return 0.0;
    }
    if len2 == 1 {
        return if len1 == 0 {
            nums2[0] as f64
        } else {
            (nums1[0] + nums2[0]) as f64 / 2.0
        };
    }

    let mut a = 0;
    let mut b = len1;
    let mut c = (len1 + 1) / 2;
    let mut d: usize;

    loop {
        d = (len1 + len2 + 1) / 2 - c;
        if c > 0 && d < len2 && nums1[c - 1] > nums2[d] {
            b = c;
            c = a + (b - a) / 2;
            continue;
        }
        if c < len1 && d > 0 && nums2[d - 1] > nums1[c] {
            a = c;
            c = a + (b - a + 1) / 2;
            continue;
        }
        break;
    }

    let (x, y) = if len1 == 0 {
        (nums2[d - 1], nums2[d])
    } else if c == 0 {
        if d == len2 {
            (nums2[d - 1], nums1[c])
        } else {
            (nums2[d - 1], min(nums1[c], nums2[d]))
        }
    } else if c == len1 {
        if d == 0 {
            (nums1[c - 1], nums2[d])
        } else {
            (max(nums1[c - 1], nums2[d - 1]), nums2[d])
        }
    } else {
        (max(nums1[c - 1], nums2[d - 1]), min(nums1[c], nums2[d]))
    };

    if (len1 + len2) % 2 == 1 {
        x as f64
    } else {
        (x + y) as f64 / 2.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(input: (Vec<i32>, Vec<i32>), expected: f64) {
        let (nums1, nums2) = input;
        let output = find_median_sorted_arrays(nums1, nums2);
        assert_eq!(output, expected);
    }

    #[test]
    fn example() {
        tf((vec![1, 3], vec![2]), 2.0);
        tf((vec![1, 2], vec![3, 4]), 2.5);
    }
}
