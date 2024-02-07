pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
    let mut feq = vec![0; 10001];
    let mut max = 0;
    nums.into_iter().for_each(|v| {
        feq[v as usize] += 1;
        if v > max {
            max = v
        }
    });

    let mut prev2 = 0;
    let mut prev1 = feq[1];
    for i in 2..=max {
        let a = prev2 + i * feq[i as usize]; // earn i-2 and i
        let b = prev1; // earn i-1
        let curr = i32::max(a, b);
        prev2 = prev1;
        prev1 = curr;
    }
    prev1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(delete_and_earn(vec![3, 4, 2]), 6);
        assert_eq!(delete_and_earn(vec![2, 2, 3, 3, 3, 4]), 9);
    }
}
