pub fn ways_to_buy_pens_pencils(total: i32, cost1: i32, cost2: i32) -> i64 {
    let mut count: i64 = 0;
    let mut n = total;
    let a = i32::max(cost1, cost2);
    let b = i32::min(cost1, cost2);
    while n >= a {
        count += (n / b) as i64 + 1;
        n -= a;
    }
    count += (n / b) as i64 + 1;
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(ways_to_buy_pens_pencils(20, 5, 10), 9);
        assert_eq!(ways_to_buy_pens_pencils(5, 10, 10), 1);

        assert_eq!(ways_to_buy_pens_pencils(10, 99, 9), 2);
    }
}
