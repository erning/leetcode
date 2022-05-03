pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    if n < 2 {
        return 0;
    }

    let mut a = 0; // the smallest index not in place
    let mut b = 0; // the largest index not in place

    let mut next = nums[n - 1];
    for (i, &curr) in nums.iter().enumerate().rev().skip(1) {
        if curr > next {
            a = i;
        } else {
            next = curr;
        }
    }
    let mut prev = nums[0];
    for (i, &curr) in nums.iter().enumerate().skip(1) {
        if curr < prev {
            b = i;
        } else {
            prev = curr;
        }
    }

    return if a == b { 0 } else { (b + 1 - a) as i32 };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(find_unsorted_subarray(vec![2, 6, 4, 8, 10, 9, 15]), 5);
        assert_eq!(find_unsorted_subarray(vec![1, 2, 3, 4]), 0);
        assert_eq!(find_unsorted_subarray(vec![1]), 0);

        assert_eq!(find_unsorted_subarray(vec![0]), 0);
        assert_eq!(find_unsorted_subarray(vec![1, 2]), 0);
        assert_eq!(find_unsorted_subarray(vec![2, 1]), 2);
        assert_eq!(find_unsorted_subarray(vec![1, 3, 2, 4]), 2);

        assert_eq!(find_unsorted_subarray(vec![1, 4, 2, 3, 5]), 3);
        assert_eq!(find_unsorted_subarray(vec![2, 4, 1, 3, 5]), 4);
        assert_eq!(find_unsorted_subarray(vec![2, 3, 1, 6, 4, 5]), 6);

        assert_eq!(
            find_unsorted_subarray(vec![5, 20, 30, 10, 0, 70, 60, 40, 50]),
            9
        );

        assert_eq!(find_unsorted_subarray(vec![1, 2, 2, 4]), 0);
    }
}
