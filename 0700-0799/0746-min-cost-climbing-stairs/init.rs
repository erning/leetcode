pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
    let mut prev2 = 0;
    let mut prev1 = 0;
    for i in 2..=cost.len() {
        let curr = i32::min(prev2 + cost[i - 2], prev1 + cost[i - 1]);
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
        assert_eq!(min_cost_climbing_stairs(vec![10, 15, 20]), 15);
        assert_eq!(
            min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1]),
            6
        );
    }
}
