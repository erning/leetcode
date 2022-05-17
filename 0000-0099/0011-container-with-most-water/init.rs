pub fn max_area(height: Vec<i32>) -> i32 {
    let mut max = 0;
    let mut iter = height.iter().enumerate();
    let mut lo = iter.next();
    let mut hi = iter.next_back();
    while let (Some((x1, &h1)), Some((x2, &h2))) = (lo, hi) {
        let x1 = x1 as i32;
        let x2 = x2 as i32;
        let area = (x2 - x1) * i32::min(h1, h2);
        if area > max {
            max = area;
        }
        if h1 < h2 {
            lo = iter.next();
        } else {
            hi = iter.next_back();
        }
    }
    max
}

#[allow(dead_code)]
fn naive(height: Vec<i32>) -> i32 {
    let mut max = 0;
    for (x1, h1) in height.iter().take(height.len() - 1).enumerate() {
        for (x2, h2) in height.iter().enumerate().skip(x1 + 1) {
            let area = (x2 as i32 - x1 as i32) * i32::min(*h1, *h2);
            if area > max {
                max = area;
            }
        }
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
        assert_eq!(max_area(vec![1, 1]), 1);
    }
}
