use std::cmp::Ordering;

pub fn my_sqrt(x: i32) -> i32 {
    let mut a = 1;
    let mut b = x;
    while a <= b {
        let c = a + (b - a) / 2;
        println!("{:?}", (a, b, c));
        match (x / c).cmp(&c) {
            Ordering::Less => b = c - 1,
            Ordering::Greater => a = c + 1,
            Ordering::Equal => {
                return c;
            }
        }
    }
    a - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(my_sqrt(4), 2);
        assert_eq!(my_sqrt(8), 2);

        assert_eq!(my_sqrt(1024), 32);
        assert_eq!(my_sqrt(2147395600), 46340);

        assert_eq!(my_sqrt(10), 3);
        assert_eq!(my_sqrt(100), 10);
    }
}
