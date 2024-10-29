use std::cmp::Ordering;

pub fn get_max_len(nums: Vec<i32>) -> i32 {
    let mut max = 0;
    let mut a = 0;
    let mut b = 0;
    for n in nums {
        match n.cmp(&0) {
            Ordering::Equal => {
                a = 0;
                b = 0;
            }
            Ordering::Greater => {
                a += 1;
                b = if b == 0 { 0 } else { b + 1 };
            }
            Ordering::Less => {
                let c = a;
                a = if b == 0 { 0 } else { b + 1 };
                b = c + 1;
            }
        }
        if a > max {
            max = a;
        }
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(get_max_len(vec![1, -2, -3, 4]), 4);
        assert_eq!(get_max_len(vec![0, 1, -2, -3, -4]), 3);
        assert_eq!(get_max_len(vec![-1, -2, -3, 0, 1]), 2);

        assert_eq!(get_max_len(vec![-1, 2]), 1);
        assert_eq!(get_max_len(vec![-1]), 0);

        assert_eq!(get_max_len(vec![-16, 0, -5, 2, 2, -13, 11, 8]), 6);
        assert_eq!(
            get_max_len(vec![
                5, -20, -20, -39, -5, 0, 0, 0, 36, -32, 0, -7, -10, -7, 21, 20, -12, -34, 26, 2
            ]),
            8
        );
    }
}
