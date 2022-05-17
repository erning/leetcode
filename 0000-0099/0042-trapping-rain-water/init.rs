pub fn trap(height: Vec<i32>) -> i32 {
    #[allow(clippy::while_let_on_iterator)]
    fn area<'a, T>(mut iter: T) -> i32
    where
        T: Iterator<Item = &'a i32>,
    {
        let mut area = 0;
        let mut prev = iter.next().unwrap();
        while let Some(h) = iter.next() {
            if h > prev {
                prev = h;
            } else {
                area += prev - h;
            }
        }
        area
    }

    let len = height.len();
    if len < 3 {
        return 0;
    }
    let heightest = height
        .iter()
        .enumerate()
        .max_by(|x, y| x.1.cmp(y.1))
        .unwrap();

    if heightest.0 == 0 {
        area(height.iter().rev())
    } else if heightest.0 == len {
        area(height.iter())
    } else {
        area(height[..heightest.0 + 1].iter()) + area(height[heightest.0..].iter().rev())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
        assert_eq!(trap(vec![4, 2, 0, 3, 2, 5]), 9);

        assert_eq!(trap(vec![4, 2, 3]), 1);
        assert_eq!(trap(vec![8, 5, 4, 1, 8, 9, 3, 0, 0]), 14);
    }
}
