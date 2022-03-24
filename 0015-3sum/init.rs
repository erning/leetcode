use std::collections::HashSet;

pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut rv: Vec<Vec<i32>> = vec![];
    let len = nums.len();
    if len < 3 {
        return rv;
    }
    let mut nums = nums.to_vec();
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
            } else if sum > 0 {
                opt_c = iter.next_back();
            } else {
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
        }
        opt_a = iter_a.next();
        while opt_a.is_some() && opt_a.unwrap().1 == a {
            opt_a = iter_a.next();
        }
    }

    rv
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
