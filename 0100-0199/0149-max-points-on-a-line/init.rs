use std::collections::HashMap;
use std::collections::HashSet;

pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
    fn gcd(a: i32, b: i32, c: i32) -> i32 {
        fn gcd(x: i32, y: i32) -> i32 {
            if x == 0 {
                return y;
            } else if y == 0 {
                return x;
            }
            if x % y != 0 {
                gcd(y, x % y)
            } else {
                y
            }
        }
        let d = if a < b { gcd(b, a) } else { gcd(a, b) };
        if c < d {
            gcd(d, c)
        } else {
            gcd(c, d)
        }
    }

    let mut map: HashMap<(i32, i32, i32), HashSet<(i32, i32)>> = HashMap::new();
    for (i, p1) in points.iter().enumerate().take(points.len() - 1) {
        for p2 in points.iter().skip(i + 1) {
            let x1 = p1[0];
            let y1 = p1[1];
            let x2 = p2[0];
            let y2 = p2[1];
            let mut a = y2 - y1;
            let mut b = x1 - x2;
            let mut c = x2 * y1 - x1 * y2;
            let d = gcd(a.abs(), b.abs(), c.abs());
            a /= d;
            b /= d;
            c /= d;
            let k = (a, b, c);
            if let Some(v) = map.get_mut(&k) {
                v.insert((x1, y1));
                v.insert((x2, y2));
            } else {
                let mut v: HashSet<(i32, i32)> = HashSet::new();
                v.insert((x1, y1));
                v.insert((x2, y2));
                map.insert(k, v);
            }
        }
    }
    map.into_values().map(|v| v.len()).max().unwrap_or(1) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(max_points(vec![vec![1, 1], vec![2, 2], vec![3, 3]]), 3);
        assert_eq!(
            max_points(vec![
                vec![1, 1],
                vec![3, 2],
                vec![5, 3],
                vec![4, 1],
                vec![2, 3],
                vec![1, 4]
            ]),
            4
        );
        assert_eq!(max_points(vec![vec![1, 1]]), 1);
        assert_eq!(max_points(vec![vec![2, 3], vec![3, 3], vec![5, 3]]), 3);
    }
}
