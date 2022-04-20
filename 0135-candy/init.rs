pub fn candy(ratings: Vec<i32>) -> i32 {
    let n = ratings.len();
    if n == 1 {
        return 1;
    }

    let mut count = 0;
    let mut peak = 1;
    let mut m = 0;

    for i in 1..n {
        let prev = (ratings[i] - ratings[i - 1]).signum();
        if i < n - 1 {
            let curr = (ratings[i + 1] - ratings[i]).signum();
            if prev == curr {
                continue;
            }
        }
        match prev {
            1 => {
                // Up -> Flat | Up -> Down
                let n = i - m;
                let c = n * (n + 1) / 2;
                count += c;
                peak = n + 1;
            }
            0 => {
                // Flat -> Down | Flat -> Up
                let n = i - m;
                let c = n + usize::max(peak, 1) - 1;
                count += c;
                peak = 1;
            }
            -1 => {
                // Down -> Flat | Down -> Up
                let n = i - m;
                let c = n * (n + 1) / 2 + usize::max(peak, n + 1) - 1;
                count += c;
                peak = 1;
            }
            _ => unimplemented!(),
        }
        m = i;
    }
    count += peak;

    count as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(candy(vec![1, 0, 2]), 5);
        assert_eq!(candy(vec![1, 2, 2]), 4);

        assert_eq!(candy(vec![1, 3, 2, 2, 1]), 7);
        assert_eq!(candy(vec![1, 2, 3, 1, 0]), 9);
        assert_eq!(candy(vec![1, 3, 2, 1, 0]), 11);
        assert_eq!(candy(vec![1, 3, 2, 2, 2, 1, 0]), 11);
        assert_eq!(candy(vec![1, 2, 3, 1, 0, 1, 2, 3, 1, 0]), 21);

        assert_eq!(candy(vec![1]), 1);
        assert_eq!(candy(vec![1, 1]), 2);
        assert_eq!(candy(vec![1, 2]), 3);
        assert_eq!(candy(vec![2, 1]), 3);
    }
}
