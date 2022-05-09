pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
    #[inline]
    fn push_answer(a: i32, b: i32, answer: &mut Vec<String>) {
        if a == b {
            answer.push(a.to_string());
        } else {
            answer.push(format!("{}->{}", a, b));
        }
    }

    let mut answer: Vec<String> = Vec::new();
    if nums.is_empty() {
        return answer;
    }
    let mut a = nums[0];
    let mut b = a;
    for n in nums.into_iter().skip(1) {
        if b + 1 == n {
            b = n;
            continue;
        }
        push_answer(a, b, &mut answer);
        a = n;
        b = n;
    }
    push_answer(a, b, &mut answer);
    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            summary_ranges(vec![0, 1, 2, 4, 5, 7]),
            &["0->2", "4->5", "7"]
        );
        assert_eq!(
            summary_ranges(vec![0, 2, 3, 4, 6, 8, 9]),
            &["0", "2->4", "6", "8->9"]
        );

        assert_eq!(summary_ranges(vec![0]), &["0"]);
        assert_eq!(summary_ranges(vec![]), Vec::<String>::new());
    }
}
