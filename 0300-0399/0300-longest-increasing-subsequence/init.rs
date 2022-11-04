pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    let mut dp: Vec<i32> = Vec::new();
    for a in nums.into_iter() {
        if let Err(i) = dp.binary_search(&a) {
            if i == dp.len() {
                dp.push(a)
            } else {
                dp[i] = a;
            }
        }
    }
    dp.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! tf {
        ($input:expr, $expected:expr) => {
            let output = length_of_lis($input.to_vec());
            assert_eq!(output, $expected, "{:?}", $input);
        };
    }

    #[test]
    fn example() {
        tf!([10, 9, 2, 5, 3, 7, 101, 18], 4);
        tf!([0, 1, 0, 3, 2, 3], 4);
        tf!([7, 7, 7, 7, 7, 7, 7], 1);

        tf!([0, 1, 0, 3, 2, 3, -1, 0], 4);
    }
}
