pub fn judge_point24(cards: Vec<i32>) -> bool {
    fn judge_point24(cards: &[f64]) -> bool {
        let count = cards.len();
        if count == 1 {
            return f64::abs(24.0 - cards[0]) <= 1e-9;
        }
        for i in 0..count - 1 {
            for j in i + 1..count {
                let (a, b) = (cards[i], cards[j]);
                let nums: Vec<f64> = cards
                    .iter()
                    .enumerate()
                    .filter(|(p, _)| p != &i && p != &j)
                    .map(|(_, &v)| v)
                    .collect();

                macro_rules! judge {
                    ($v:expr) => {
                        let mut nums = nums.clone();
                        nums.push($v);
                        if judge_point24(&nums) {
                            return true;
                        }
                    };
                }

                judge!(a + b);
                judge!(a - b);
                judge!(b - a);
                judge!(a * b);
                if b != 0.0 {
                    judge!(a / b);
                }
                if a != 0.0 {
                    judge!(b / a);
                }
            }
        }
        false
    }
    let cards: Vec<f64> = cards.iter().map(|v| *v as f64).collect();
    judge_point24(&cards)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(input: Vec<i32>, expected: bool) {
        let output = judge_point24(input);
        assert_eq!(output, expected);
    }

    #[test]
    fn example() {
        tf(vec![4, 1, 8, 7], true);
        tf(vec![1, 2, 1, 2], false);
        tf(vec![1, 5, 5, 5], true);
        tf(vec![7, 7, 3, 3], true);
        tf(vec![1, 1, 1, 7], false);
    }
}
