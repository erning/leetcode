// https://youtu.be/6zfXmg9Btao
pub fn count_digit_one(n: i32) -> i32 {
    if n <= 0 {
        return 0;
    }
    let mut m = 1;
    let mut sum = 0;
    while m <= n {
        let lhs = n / m / 10;
        let cur = n / m % 10;
        let rhs = n % m;
        if cur > 1 {
            sum += (lhs + 1) * m;
        } else if cur < 1 {
            sum += lhs * m;
        } else {
            sum += lhs * m + rhs + 1;
        }
        m *= 10;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(count_digit_one(13), 6);
        assert_eq!(count_digit_one(0), 0);
    }
}
