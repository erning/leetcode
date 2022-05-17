pub fn maximum_gap(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    if n <= 1 {
        return 0;
    }
    let mut min = nums[0];
    let mut max = nums[0];
    for &v in nums.iter().skip(1) {
        if v < min {
            min = v;
        } else if v > max {
            max = v;
        }
    }

    let stride = 1 + (max - min) / (n - 1) as i32;
    let mut buckets: Vec<Option<(i32, i32)>> = vec![None; ((max - min) / stride + 1) as usize];
    for &v in nums.iter() {
        let i = ((v - min) / stride) as usize;
        if let Some((min, max)) = &mut buckets[i] {
            if v < *min {
                *min = v;
            } else if v > *max {
                *max = v;
            }
        } else {
            buckets[i] = Some((v, v));
        }
    }

    let mut gap = 0;
    let mut prev: Option<(i32, i32)> = None;
    for curr in buckets.into_iter().filter(|v| v.is_some()) {
        if let Some(v) = prev {
            let diff = curr.unwrap().0 - v.1;
            if diff > gap {
                gap = diff
            }
        } else {
            gap = curr.unwrap().1 - curr.unwrap().0;
        }
        // if prev.is_some() {
        //     let diff = curr.unwrap().0 - prev.unwrap().1;
        //     if diff > gap {
        //         gap = diff;
        //     }
        // } else {
        //     gap = curr.unwrap().1 - curr.unwrap().0;
        // }
        prev = curr;
    }
    gap
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(maximum_gap(vec![3, 6, 9, 1]), 3);
        assert_eq!(maximum_gap(vec![10]), 0);

        assert_eq!(maximum_gap(vec![1, 10000000]), 9999999);
    }
}
