pub fn count_lattice_points(circles: Vec<Vec<i32>>) -> i32 {
    let x1 = 0;
    let y1 = 0;
    let x2 = 200;
    let y2 = 200;

    let mut count = 0;
    for y in y1..=y2 {
        for x in x1..=x2 {
            for circle in circles.iter() {
                let dx = (x - circle[0]).abs();
                let dy = (y - circle[1]).abs();
                let r = circle[2];
                if dx * dx + dy * dy <= r * r {
                    count += 1;
                    break;
                }
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf(input: &[[i32; 3]], excepted: i32) {
        let circles: Vec<Vec<i32>> = input.into_iter().map(|v| v.to_vec()).collect();
        let output = count_lattice_points(circles);
        assert_eq!(output, excepted, "{:?}", input);
    }

    #[test]
    fn example() {
        tf(&[[2, 2, 1]], 5);
        tf(&[[2, 2, 2], [3, 4, 1]], 16);

        tf(&[[8,9,6],[9,8,4],[4,1,1],[8,5,1],[7,1,1],[6,7,5],[7,1,1],[7,1,1],[5,5,3]], 141);
    }
}
