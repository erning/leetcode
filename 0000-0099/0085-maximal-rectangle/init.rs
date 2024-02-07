pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
    fn build_histogram(matrix: &[Vec<char>]) -> Vec<i32> {
        let mut histogram: Vec<i32> = vec![0; matrix[0].len()];
        let mut ignores: Vec<bool> = matrix[0].iter().map(|&c| c == '0').collect();
        for row in matrix.iter() {
            for (x, v) in histogram.iter_mut().enumerate() {
                if ignores[x] {
                    continue;
                }
                if row[x] == '1' {
                    *v += 1;
                } else {
                    ignores[x] = true;
                }
            }
        }
        histogram
    }

    fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
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

    let mut max = 0;
    for i in 0..matrix.len() {
        let histogram = build_histogram(&matrix[i..]);
        let area = largest_rectangle_area(histogram);
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
        assert_eq!(
            maximal_rectangle(vec![
                vec!['1', '0', '1', '0', '0'],
                vec!['1', '0', '1', '1', '1'],
                vec!['1', '1', '1', '1', '1'],
                vec!['1', '0', '0', '1', '0'],
            ]),
            6
        );
        assert_eq!(maximal_rectangle(vec![vec!['0']]), 0);
        assert_eq!(maximal_rectangle(vec![vec!['1']]), 1);

        maximal_rectangle(vec![
            vec!['1', '0', '1', '1', '1'],
            vec!['0', '1', '0', '1', '0'],
            vec!['1', '1', '0', '1', '1'],
            vec!['1', '1', '0', '1', '1'],
            vec!['0', '1', '1', '1', '1'],
        ]);
    }
}
