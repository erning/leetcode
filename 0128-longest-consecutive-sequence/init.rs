use std::collections::HashSet;

pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let mut remains: HashSet<i32> = nums.iter().map(|v| *v).collect();
    let mut longest = 0;
    while let Some(&n) = remains.iter().next() {
        remains.remove(&n);
        let mut m = n + 1;
        while let Some(_) = remains.take(&m) {
            m += 1;
        }
        let mut count = m - n;
        m = n - 1;
        while let Some(_) = remains.take(&m) {
            m -= 1;
        }
        count += n - m - 1;
        if count > longest {
            longest = count;
        }
    }
    longest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(longest_consecutive(vec![4, 3, 2, 1]), 4);
        assert_eq!(longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
        assert_eq!(longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]), 9);
    }
}
