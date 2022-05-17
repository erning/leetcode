use std::collections::HashSet;

pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let len = nums.len();
    let mut nums = nums;
    nums.sort_unstable();

    let mut rv: HashSet<Vec<i32>> = HashSet::new();
    let mut iter_a = nums.iter().take(len - 3).enumerate();
    let mut opt_a = iter_a.next();
    while let Some((i, a)) = opt_a {
        let mut iter_b = nums.iter().take(len - 2).enumerate().skip(i + 1);
        let mut opt_b = iter_b.next();
        while let Some((j, b)) = opt_b {
            let mut iter = nums.iter().skip(j + 1);
            let mut lo = iter.next();
            let mut hi = iter.next_back();
            while let (Some(c), Some(d)) = (lo, hi) {
                let sum = a + b + c + d;
                if sum < target {
                    lo = iter.next();
                    continue;
                }
                if sum > target {
                    hi = iter.next_back();
                    continue;
                }
                rv.insert(vec![*a, *b, *c, *d]);
                lo = iter.next();
                while lo.is_some() && lo.unwrap() == c {
                    lo = iter.next();
                }
                hi = iter.next_back();
                while hi.is_some() && hi.unwrap() == d {
                    hi = iter.next_back();
                }
            }
            opt_b = iter_b.next();
            while opt_b.is_some() && opt_b.unwrap().1 == b {
                opt_b = iter_b.next();
            }
        }
        opt_a = iter_a.next();
        while opt_a.is_some() && opt_a.unwrap().1 == a {
            opt_a = iter_a.next();
        }
    }
    rv.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(nums: Vec<i32>, target: i32, expected: Vec<Vec<i32>>) {
        let mut output = four_sum(nums, target);
        output.sort_unstable();
        assert_eq!(output, expected);
    }

    #[test]
    fn example() {
        tf(
            vec![1, 0, -1, 0, -2, 2],
            0,
            vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]],
        );
        tf(vec![2, 2, 2, 2, 2], 8, vec![vec![2, 2, 2, 2]]);

        tf(vec![0, 0, 0], 0, vec![]);
        tf(vec![-3, -1, 0, 2, 4, 5], 0, vec![vec![-3, -1, 0, 4]]);
        tf(
            vec![-3, -2, -1, 0, 0, 1, 2, 3],
            0,
            vec![
                vec![-3, -2, 2, 3],
                vec![-3, -1, 1, 3],
                vec![-3, 0, 0, 3],
                vec![-3, 0, 1, 2],
                vec![-2, -1, 0, 3],
                vec![-2, -1, 1, 2],
                vec![-2, 0, 0, 2],
                vec![-1, 0, 0, 1],
            ],
        );
        tf(
            vec![
                2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
                2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
                2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
                2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
                2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
                2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
                2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
                2, 2, 2, 2,
            ],
            8,
            vec![vec![2, 2, 2, 2]],
        )
    }
}
