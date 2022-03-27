pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
    let len = nums.len();
    let mut nums = nums;
    nums.sort_unstable();

    let mut min = (i32::MAX, i32::MAX);
    let mut iter_a = nums.iter().take(len - 2).enumerate();
    let mut opt_a = iter_a.next();
    while let Some((i, a)) = opt_a {
        let mut iter = nums[i + 1..].iter();
        let mut opt_b = iter.next();
        let mut opt_c = iter.next_back();
        while let (Some(b), Some(c)) = (opt_b, opt_c) {
            let sum = a + b + c;
            let diff = sum - target;
            if diff < 0 {
                if min.0 > -diff {
                    min = (-diff, sum);
                }
                opt_b = iter.next();
                while opt_b.is_some() && opt_b.unwrap() == b {
                    opt_b = iter.next();
                }
                continue;
            }
            if diff > 0 {
                if min.0 > diff {
                    min = (diff, sum);
                }
                opt_c = iter.next_back();
                while opt_c.is_some() && opt_c.unwrap() == c {
                    opt_c = iter.next_back();
                }
                continue;
            }
            return sum;
        }
        opt_a = iter_a.next();
        while opt_a.is_some() && opt_a.unwrap().1 == a {
            opt_a = iter_a.next();
        }
    }

    min.1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
        assert_eq!(three_sum_closest(vec![0, 0, 0], 1), 0);

        assert_eq!(three_sum_closest(vec![0, 2, 1, -3], 1), 0);
        assert_eq!(three_sum_closest(vec![1, 2, 4, 8, 16, 32, 64, 128], 82), 82);
    }
}
