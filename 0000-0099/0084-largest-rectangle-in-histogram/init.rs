pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
    let mut stack: Vec<i32> = Vec::new();
    let mut max = 0;

    for (i, &h) in heights.iter().enumerate() {
        let i = i as i32;
        loop {
            let prev = stack.last();
            if prev.is_none() {
                break;
            }
            let &prev_i = prev.unwrap();
            let prev_h = heights[prev_i as usize];
            if h > prev_h {
                break;
            }
            stack.pop();
            let w = i - *stack.last().unwrap_or(&-1) - 1;
            let area = prev_h * w;
            if area > max {
                max = area;
            }
        }
        stack.push(i);
    }

    let len = heights.len() as i32;
    while let Some(i) = stack.pop() {
        let w = len - *stack.last().unwrap_or(&-1) - 1;
        let h = heights[i as usize];
        let area = h * w;
        if area > max {
            max = area;
        }
    }

    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]), 10);
        assert_eq!(largest_rectangle_area(vec![2, 4]), 4);

        assert_eq!(largest_rectangle_area(vec![1, 1, 1, 1, 1]), 5);
        assert_eq!(largest_rectangle_area(vec![2, 1, 2]), 3);

        assert_eq!(largest_rectangle_area(vec![5, 4, 1, 2]), 8);
        assert_eq!(largest_rectangle_area(vec![4, 2, 0, 3, 2, 5]), 6);
    }
}
