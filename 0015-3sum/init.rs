pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut rv: Vec<Vec<i32>> = vec![];
    let len = nums.len();
    if len < 3 {
        return rv;
    }
    let mut nums = nums;
    nums.sort_unstable();

    let mut iter_a = nums.iter().take(len - 2).enumerate();
    let mut opt_a = iter_a.next();
    while let Some((i, a)) = opt_a {
        let mut iter = nums[i + 1..].iter();
        let mut opt_b = iter.next();
        let mut opt_c = iter.next_back();
        while let (Some(b), Some(c)) = (opt_b, opt_c) {
            let sum = a + b + c;
            if sum < 0 {
                opt_b = iter.next();
                continue;
            }
            if sum > 0 {
                opt_c = iter.next_back();
                continue;
            }
            rv.push(vec![*a, *b, *c]);
            opt_b = iter.next();
            while opt_b.is_some() && opt_b.unwrap() == b {
                opt_b = iter.next();
            }
            opt_c = iter.next_back();
            while opt_c.is_some() && opt_c.unwrap() == c {
                opt_c = iter.next_back();
            }
        }
        opt_a = iter_a.next();
        while opt_a.is_some() && opt_a.unwrap().1 == a {
            opt_a = iter_a.next();
        }
    }

    rv

    // let mut i = 0;
    // while i < len - 2 {
    //     let a = nums[i];
    //     let mut lo = i + 1;
    //     let mut hi = len - 1;
    //     while lo < hi {
    //         let b = nums[lo];
    //         let c = nums[hi];
    //         let sum = a + b + c;
    //         if sum < 0 {
    //             lo += 1;
    //             continue;
    //         }
    //         if sum > 0 {
    //             hi -= 1;
    //             continue;
    //         }
    //         rv.push(vec![a, b, c]);
    //         lo += 1;
    //         while lo < hi && nums[lo] == b {
    //             lo += 1;
    //         }
    //         hi -= 1;
    //         while lo < hi && nums[hi] == c {
    //             hi -= 1;
    //         }
    //     }
    //     i += 1;
    //     while i < len - 2 && nums[i] == a {
    //         i += 1;
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            three_sum(vec![-1, 0, 1, 2, -1, -4]),
            vec![vec![-1, -1, 2], vec![-1, 0, 1]]
        );
        let empty: Vec<Vec<i32>> = vec![];
        assert_eq!(three_sum(vec![]), empty);
        assert_eq!(three_sum(vec![0]), empty);
    }
}
