pub fn trap(height: Vec<i32>) -> i32 {
    fn recursion(height: &[i32]) -> i32 {
        if height.len() < 3 {
            return 0;
        }
        let a = (0, height[0]);
        let b = (height.len() - 1, height[height.len() - 1]);
        let c = height
            .iter()
            .enumerate()
            .take(height.len() - 1)
            .skip(1)
            .map(|(i, &v)| (i, v))
            .max_by(|x, y| x.1.cmp(&y.1))
            .unwrap();

        let area = |x: (usize, i32), y: (usize, i32)| -> i32 {
            let h = i32::min(x.1, y.1);
            let w = y.0 as i32 - x.0 as i32 - 1;
            h * w - height[x.0 + 1..y.0].iter().sum::<i32>()
        };

        if c.1 <= a.1 && c.1 <= b.1 {
            return area(a, b);
        }
        if c.1 <= a.1 {
            return area(a, c) + recursion(&height[c.0..b.0 + 1]);
        }
        if c.1 <= b.1 {
            return area(c, b) + recursion(&height[a.0..c.0 + 1]);
        }
        return recursion(&height[a.0..c.0 + 1]) + recursion(&height[c.0..b.0 + 1]);
    }
    recursion(&height[..])
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
