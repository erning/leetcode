pub fn count_rectangles(rectangles: Vec<Vec<i32>>, points: Vec<Vec<i32>>) -> Vec<i32> {
    let mut rows: Vec<Vec<(i32, i32)>> = vec![Vec::new(); 101];
    for p in rectangles.iter() {
        let x = p[0];
        let y = p[1];
        rows[y as usize].push((x, y));
    }
    let mut rows: Vec<(i32, Vec<i32>)> = rows
        .into_iter()
        .enumerate()
        .filter(|(_, v)| !v.is_empty())
        .map(|(i, v)| (i as i32, v.into_iter().map(|(x, _)| x).collect()))
        .collect();

    for i in (0..rows.len() - 1).rev() {
        let mut ext = rows[i + 1].1.clone();
        rows[i].1.append(&mut ext);
        rows[i].1.sort_unstable();
    }
    if let Some((_, row)) = rows.last_mut() {
        row.sort();
    }

    let mut answer = Vec::with_capacity(points.len());
    for p in points.into_iter() {
        let x = p[0];
        let y = p[1];
        let i = match rows.binary_search_by_key(&y, |v| v.0) {
            Ok(v) | Err(v) => v,
        };
        if i >= rows.len() {
            answer.push(0);
            continue;
        }
        let row = &rows[i].1;
        let mut j = match row.binary_search(&x) {
            Ok(v) | Err(v) => v,
        };
        if j >= row.len() {
            answer.push(0);
            continue;
        }
        let v = row[j];
        while j > 0 && row[j - 1] == v {
            j -= 1;
        }
        let v = row.len() - j;
        answer.push(v as i32);
    }
    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(rectangles: &[[i32; 2]], points: &[[i32; 2]], expected: &[i32]) {
        let r: Vec<Vec<i32>> = rectangles.iter().map(|v| v.to_vec()).collect();
        let p: Vec<Vec<i32>> = points.iter().map(|v| v.to_vec()).collect();
        let output = count_rectangles(r, p);
        assert_eq!(output, expected, "{:?}", (rectangles, points));
    }

    #[test]
    fn example() {
        tf(&[[1, 2], [2, 3], [2, 5]], &[[2, 1], [1, 4]], &[2, 1]);
        tf(&[[1, 1], [2, 2], [3, 3]], &[[1, 3], [1, 1]], &[1, 3]);

        tf(
            &[[4, 7], [4, 9], [8, 5], [6, 2], [6, 4]],
            &[[4, 2], [5, 6]],
            &[5, 0],
        );

        tf(
            &[[7, 1], [2, 6], [1, 4], [5, 2], [10, 3], [2, 4], [5, 9]],
            &[
                [10, 3],
                [8, 10],
                [2, 3],
                [5, 4],
                [8, 5],
                [7, 10],
                [6, 6],
                [3, 6],
            ],
            &[1, 0, 4, 1, 0, 0, 0, 1],
        );
    }
}
