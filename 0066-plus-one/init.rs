pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut rv = Vec::with_capacity(digits.len() + 1);
    let mut c = 1;
    for v in digits.into_iter().rev() {
        let v = v + c;
        c = v / 10;
        rv.push(v % 10);
    }
    if c > 0 {
        rv.push(c);
    }
    rv.reverse();
    rv
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
        assert_eq!(plus_one(vec![9]), vec![1, 0]);
    }
}
