pub fn largest_number(nums: Vec<i32>) -> String {
    let mut nums: Vec<String> = nums.into_iter().map(|v| format!("{}", v)).collect();
    nums.sort_unstable_by(|a, b| {
        let v1 = format!("{}{}", a, b);
        let v2 = format!("{}{}", b, a);
        v2.cmp(&v1)
    });
    for (i, s) in nums.iter().enumerate() {
        if s != "0" {
            return nums[i..].join("");
        }
    }
    "0".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(largest_number(vec![10, 2]), "210");
        assert_eq!(largest_number(vec![3, 30, 34, 5, 9]), "9534330");

        assert_eq!(largest_number(vec![0, 0]), "0");
    }
}
