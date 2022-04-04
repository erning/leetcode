pub fn trap(height: Vec<i32>) -> i32 {
    if height.len() < 3 {
        return 0;
    }
    let mut h1 = -1;
    let mut h2 = -1;
    let mut i1 = 0;
    let mut i2 = 0;
    for (i, &h) in height.iter().enumerate() {
        if h > h1 {
            h2 = h1;
            i2 = i1;
            h1 = h;
            i1 = i;
        } else if h > h2 {
            h2 = h;
            i2 = i;
        }
    }

    let (a, b) = if i1 < i2 { (i1, i2) } else { (i2, i1) };
    let mut area = 0;

    if h2 > 0 {
        let w = b as i32 - a as i32 - 1;
        if w > 0 {
            area += w * h2 - height[a + 1..b].iter().sum::<i32>()
        }
    }
    if a > 1 {
        area += trap(height[..a + 1].to_vec());
    }
    if b < height.len() - 2 {
        area += trap(height[b..].to_vec());
    }

    area
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
