use std::cmp::Ordering;

pub fn my_sqrt(x: i32) -> i32 {
    let mut n = x;
    let mut stack = Vec::with_capacity(5);
    while n >= 100 {
        let m = n % 100;
        n /= 100;
        stack.push(m);
    }
    let mut y = if n == 0 {
        0
    } else if n < 2 * 2 {
        1
    } else if n < 3 * 3 {
        2
    } else if n < 4 * 4 {
        3
    } else if n < 5 * 5 {
        4
    } else if n < 6 * 6 {
        5
    } else if n < 7 * 7 {
        6
    } else if n < 8 * 8 {
        7
    } else if n < 9 * 9 {
        8
    } else if n < 10 * 10 {
        9
    } else {
        unimplemented!()
    };
    while let Some(m) = stack.pop() {
        n = n * 100 + m;
        y *= 10;
        for _ in 0..10 {
            if y >= 46340 {
                // max sqrt for i32 to avoid overflow
                break;
            }
            y += 1;
            let z = y * y;
            match z.cmp(&n) {
                Ordering::Greater => {
                    y -= 1;
                    break;
                }
                Ordering::Equal => {
                    break;
                }
                _ => {}
            }
            // if z > n {
            //     y -= 1;
            //     break;
            // } else if z == n {
            //     break;
            // }
        }
    }
    y
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
