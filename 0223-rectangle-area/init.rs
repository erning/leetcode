#[allow(clippy::too_many_arguments)]
pub fn compute_area(
    ax1: i32,
    ay1: i32,
    ax2: i32,
    ay2: i32,
    bx1: i32,
    by1: i32,
    bx2: i32,
    by2: i32,
) -> i32 {
    fn intersectoin(mut a1: i32, mut a2: i32, mut b1: i32, mut b2: i32) -> i32 {
        if a1 == a2 || b1 == b2 {
            return 0;
        }
        if a1 > a2 {
            std::mem::swap(&mut a1, &mut a2);
        }
        if b1 > b2 {
            std::mem::swap(&mut b1, &mut b2);
        }
        if b2 <= a1 {
            0 // b1, b2, a1, a2
        } else if b1 <= a1 && b2 >= a1 && b2 <= a2 {
            b2 - a1 // b1, a1, b2, a2
        } else if b1 <= a1 && b2 >= a2 {
            a2 - a1 // b1, a1, a2, b2
        } else if b1 >= a1 && b2 <= a2 {
            b2 - b1 // a1, b1, b2, a2
        } else if b1 >= a1 && b1 <= a2 && b2 >= a2 {
            a2 - b1 // a1, b1, a2, b2
        } else {
            0 // a1, a2, b1, b2
        }
    }

    let adx = i32::abs(ax1 - ax2);
    let ady = i32::abs(ay1 - ay2);
    let bdx = i32::abs(bx1 - bx2);
    let bdy = i32::abs(by1 - by2);

    let ix = intersectoin(ax1, ax2, bx1, bx2);
    let iy = intersectoin(ay1, ay2, by1, by2);

    adx * ady + bdx * bdy - ix * iy
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(compute_area(-3, 0, 3, 4, 0, -1, 9, 2), 45);
        assert_eq!(compute_area(2, -2, 2, 2, -2, -2, 2, 2), 16);
    }
}
