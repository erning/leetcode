pub fn missing_number(nums: Vec<i32>) -> i32 {
    let mut set = vec![true; nums.len() + 1];
    nums.into_iter().for_each(|v| set[v as usize] = false);
    set.into_iter().enumerate().find(|v| v.1).unwrap().0 as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(missing_number(vec![3, 0, 1]), 2);
        assert_eq!(missing_number(vec![0, 1]), 2);
        assert_eq!(missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]), 8);
    }
}
