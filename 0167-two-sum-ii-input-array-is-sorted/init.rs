pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut a = 0;
    let mut b = numbers.len();
    while a < b {
        let c = numbers[a] + numbers[b - 1];
        if c < target {
            a += 1;
        } else if c > target {
            b -= 1;
        } else {
            return vec![(a + 1) as i32, b as i32];
        }
    }
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![1, 2]);
        assert_eq!(two_sum(vec![2, 3, 4], 6), vec![1, 3]);
        assert_eq!(two_sum(vec![-1, 0], -1), vec![1, 2]);
    }
}
